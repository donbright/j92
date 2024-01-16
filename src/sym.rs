use crate::quadrance;
use crate::distance;
use crate::Edge;
use crate::Face;
use crate::Point;
use crate::Polyhedron;
use crate::PseudoField;
use rug::{Integer, Rational};
use rug::ops::Pow;
use std::str::FromStr;
use std::fmt;

// This is symbolic number type
// however we alsu use RugRat Rational and Integer for certain calculations
// (these are Big types limited only by your system amount of memory)

#[derive(Clone, Debug, PartialEq)]
struct FieldSym(String);

fn to_rat(decimal_str: &String) -> Result<Rational, Box<dyn std::error::Error>> {
	
	// for decimal ascii floating point, like 1.2345
    let parts: Vec<&str> = decimal_str.split('.').collect();
    let digits_after_point = parts.get(1).map_or(0, |x| x.len());

    let numerator_str = decimal_str.replace('.', "");
    let numerator = Integer::from_str(&numerator_str)?;

    // Using rug::Integer's pow method
    let denominator = Integer::from(10).pow(digits_after_point as u32);

    Ok(Rational::from((numerator, denominator)))
}

#[test]
fn test_to_rat() {
    let a = to_rat(&"1.2".to_string()).unwrap();
    assert!(a==Rational::from((12,10)));
}

impl FromStr for FieldSym {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<FieldSym, Self::Err> {
		match to_rat(&s.to_string()) {
            Ok(num) => Ok(FieldSym(format!("{:?}",num))),
            Err(e) => Ok(FieldSym((&s).to_string())),
        }
    }
}

#[test]
fn test_sym_fromstr() {
    assert!(FieldSym::from_str("1").unwrap().to_string()=="1");
    assert!(FieldSym::from_str("a").unwrap().to_string()=="a");
}

impl fmt::Display for FieldSym {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl PseudoField<String> for FieldSym {
    fn add(&self, other: &Self) -> Self {
		match(to_rat(&self.0), to_rat(&other.0)) {
			(Ok(r1),Ok(r2))=>FieldSym(format!("{:?}",r1+r2)),
			_=>FieldSym(format!("{}+{}",self.0,other.0)),
		}
    }

    fn sub(&self, other: &Self) -> Self {
		match(to_rat(&self.0), to_rat(&other.0)) {
			(Ok(r1),Ok(r2))=>FieldSym(format!("{:?}",r1-r2)),
			_=>FieldSym(format!("{}-{}",self.0,other.0)),
		}
    }

    fn mul(&self, other: &Self) -> Self {
		match(to_rat(&self.0), to_rat(&other.0)) {
			(Ok(r1),Ok(r2))=>FieldSym(format!("{:?}",r1*r2)),
			_=>FieldSym(format!("{}{}",self.0,other.0)),
		}
    }

    fn equal(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }

    fn sqrt(&self) -> Self {
        match to_rat(&self.0) {
            Ok(n) => {
                if n.numer().is_perfect_square() && n.denom().is_perfect_square() {
                    FieldSym(format!("{}",
Rational::from( ( n.numer().clone().sqrt() , 
                  n.denom().clone().sqrt() ) )
))
                } else {
                    FieldSym(format!("√{}",self))
                }
            }
            Err(e) => {
                FieldSym(format!("√{}",self))
            }
        }
    }

    fn zero() -> Self {
        FieldSym("0".to_string())
    }

    fn one() -> Self {
        FieldSym("1".to_string())
    }
}

#[test]
fn test_field_sym_basic() {
    let a = "1.2".to_sym();
    assert_eq!(a.to_string(),"6/5".to_string());
}


#[test]
fn test_field_sym_add() {
    let a = FieldSym("1.0".to_string());
    let b = FieldSym("2.0".to_string());
    assert_eq!(a.add(&b).0, "3".to_string());
    let a2 = FieldSym("1.0".to_string());
    let b2 = FieldSym("-2.0".to_string());
    assert_eq!(a2.add(&b2).0, "-1".to_string());
    let a = FieldSym("a".to_string());
    let b = FieldSym("b".to_string());
    assert_eq!(a.add(&b).0, "a+b".to_string());
    let a = FieldSym("a".to_string());
    let b = FieldSym("2".to_string());
    assert_eq!(a.add(&b).0, "a+2".to_string());
    let a = FieldSym("2".to_string());
    let b = FieldSym("a".to_string());
    assert_eq!(a.add(&b).0, "2+a".to_string());
}

#[test]
fn test_field_sym_mul() {
    let a3 = FieldSym("4".to_string());
    let b3 = FieldSym("5".to_string());
    assert_eq!(a3.mul(&b3).0, "20".to_string());
    let a3 = FieldSym("a".to_string());
    let b3 = FieldSym("b".to_string());
    assert_eq!(a3.mul(&b3).0, "ab".to_string());
    let a3 = FieldSym("2".to_string());
    let b3 = FieldSym("b".to_string());
    assert_eq!(a3.mul(&b3).0, "2b".to_string());
}

#[test]
fn test_field_sym_sqrt() {
    let a = FieldSym("25".to_string());
    assert_eq!(a.sqrt().to_string(), "5".to_string());
    let a = FieldSym("3".to_string());
    assert_eq!(a.sqrt().to_string(), "√3".to_string());
    let a = FieldSym("a".to_string());
    assert_eq!(a.sqrt().to_string(), "√a".to_string());
}

#[derive(Clone, Debug)]
struct PointSym {
    x: FieldSym,
    y: FieldSym,
}

impl Point<String, FieldSym> for PointSym {
    fn coordinates(&self) -> Box<dyn Iterator<Item = FieldSym>> {
        Box::new(vec![self.x.clone(), self.y.clone()].into_iter())
    }
}

#[test]
fn test_point_sym() {
    let point1 = PointSym {
        x: FieldSym("1.0".to_string()),
        y: FieldSym("2.0".to_string()),
    };
    let point2 = PointSym {
        x: FieldSym("4.0".to_string()),
        y: FieldSym("6.0".to_string()),
    };

    let q = quadrance(&point1, &point2);
    let dist = distance(&point1, &point2);

    println!("Quadrance: {}", q.0);
    println!("Distance: {}", dist.0);
    assert!(dist.0 == "5");
    assert!(q.0 == "25");
}

#[derive(Debug)]
struct EdgeSym<T> {
    start: T,
    end: T,
}

impl<X, T, U> Edge<X, T, U> for EdgeSym<X>
where
    X: Point<T, U> + 'static + Copy,
    U: PseudoField<T>,
{
    fn points(&self) -> Box<dyn Iterator<Item = X>> {
        Box::new(vec![self.start, self.end].into_iter())
    }
}

trait ToSym {
    fn to_sym(&self) -> FieldSym;
}

impl ToSym for str {
    fn to_sym(&self) -> FieldSym {
        FieldSym::from_str(&self.to_string()).unwrap()
    }
}

#[test]
fn test_edge_sym() {
    let point1 = PointSym {
        x: "1".to_sym(),
        y: "2".to_sym(),
    };
    let point2 = PointSym {
        x: "4".to_sym(),
        y: "6".to_sym(),
    };
    let e1 = EdgeSym {
        start: point1,
        end: point2,
    };
    println!("{:?}", distance(&e1.start, &e1.end));
}

#[derive(Debug)]
struct FaceSym<T> {
    edges: Vec<T>,
}

impl<Z, X, T, U> Face<Z, X, T, U> for FaceSym<Z>
where
    Z: Edge<X, T, U> + 'static + Copy,
    X: Point<T, U> + 'static + Copy,
    U: PseudoField<T>,
{
    fn edges(&self) -> Box<dyn Iterator<Item = Z>> {
        Box::new(self.edges.clone().into_iter())
    }
}

#[test]
fn test_face_sym() {
    let point1 = PointSym {
        x: "1".to_sym(),
        y: "2".to_sym(),
    };
    let point2 = PointSym {
        x: "4".to_sym(),
        y: "6".to_sym(),
    };
    let point3 = PointSym {
        x: "0".to_sym(),
        y: "0".to_sym(),
    };

    let e1 = EdgeSym {
        start: point1.clone(),
        end: point2.clone(),
    };
    let e2 = EdgeSym {
        start: point2.clone(),
        end: point3.clone(),
    };
    let e3 = EdgeSym {
        start: point3.clone(),
        end: point1.clone(),
    };
    let f = FaceSym {
        edges: vec![e1, e2, e3],
    };
    println!("{:?}", f);
}

#[derive(Debug)]
struct PolyhedronSym<T> {
    faces: Vec<T>,
}

impl<M, Z, X, T, U> Polyhedron<M, Z, X, T, U> for PolyhedronSym<M>
where
    M: Face<Z, X, T, U> + 'static + Copy,
    Z: Edge<X, T, U> + 'static + Copy,
    X: Point<T, U> + 'static + Copy,
    U: PseudoField<T>,
{
    fn faces(&self) -> Box<dyn Iterator<Item = M>> {
        Box::new(self.faces.clone().into_iter())
    }
}

