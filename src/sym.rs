use crate::distance;
use crate::Edge;
use crate::Face;
use crate::Point;
use crate::Polyhedron;
use crate::PseudoField;
use rug::{Integer, Rational};
use rug::ops::Pow;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct FieldSym(String);

fn to_rat(decimal_str: &String) -> Result<Rational, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = decimal_str.split('.').collect();
    let digits_after_point = parts.get(1).map_or(0, |x| x.len());

    let numerator_str = decimal_str.replace('.', "");
    let numerator = Integer::from_str(&numerator_str)?;

    // Using rug::Integer's pow method
    let denominator = Integer::from(10).pow(digits_after_point as u32);

    Ok(Rational::from((numerator, denominator)))
}


impl PseudoField<String> for FieldSym {
    fn add(&self, other: &Self) -> Self {
		let rat = to_rat(&self.0).unwrap() + to_rat(&other.0).unwrap();
        FieldSym(format!("{:?}",rat))
    }

    fn sub(&self, other: &Self) -> Self {
		let rat = to_rat(&self.0).unwrap() - to_rat(&other.0).unwrap();
        FieldSym(format!("{:?}",rat))
    }

    fn mul(&self, other: &Self) -> Self {
		let rat = to_rat(&self.0).unwrap() * to_rat(&other.0).unwrap();
        FieldSym(format!("{:?}",rat))
    }

    fn sqrt(&self) -> Self {
        FieldSym(format!("√{:?}",self))
    }

    fn zero() -> Self {
        FieldSym("0".to_string())
    }

    fn one() -> Self {
        FieldSym("1".to_string())
    }
}

#[test]
fn test_field_String_add() {
    let a = FieldSym("1.0".to_string());
    let b = FieldSym("2.0".to_string());
    assert_eq!(a.add(&b).0, "3.0".to_string());
}

#[derive(Clone, Debug)]
struct PointSym {
    x: FieldSym,
    y: FieldSym,
}

impl Point<String, FieldSym> for PointSym {
    fn coordinates(&self) -> Box<dyn Iterator<Item = FieldSym>> {
        Box::new(vec![self.x, self.y].into_iter())
    }
}

#[test]
fn test_PointSym() {
    let point1 = PointSym {
        x: FieldSym("1.0".to_string()),
        y: FieldSym("2.0".to_string()),
    };
    let point2 = PointSym {
        x: FieldSym("4.0".to_string()),
        y: FieldSym("6.0".to_string()),
    };

    let dist = distance(&point1, &point2);

    println!("Distance: {}", dist.0);
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

#[test]
fn test_EdgeSym() {
    let point1 = PointSym {
        x: FieldSym(1.0),
        y: FieldSym(2.0),
    };
    let point2 = PointSym {
        x: FieldSym(4.0),
        y: FieldSym(6.0),
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
fn test_FaceSym() {
    let point1 = PointSym {
        x: FieldSym(1.0),
        y: FieldSym(2.0),
    };
    let point2 = PointSym {
        x: FieldSym(4.0),
        y: FieldSym(6.0),
    };
    let point3 = PointSym {
        x: FieldSym(0.0),
        y: FieldSym(0.0),
    };

    let e1 = EdgeSym {
        start: point1,
        end: point2,
    };
    let e2 = EdgeSym {
        start: point2,
        end: point3,
    };
    let e3 = EdgeSym {
        start: point3,
        end: point1,
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

