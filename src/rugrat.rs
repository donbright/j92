use crate::PseudoField;
use crate::Point;
use crate::distance;
use rug::Rational;

#[derive(Clone)]
struct RugRat(Rational);

impl PseudoField<RugRat> for RugRat {
    fn add(&self, other: RugRat) -> RugRat {
        RugRat(self.0.clone() + other.0.clone())
    }


    fn sub(&self, other: RugRat) -> RugRat {
        RugRat(self.0.clone() - other.0.clone())
    }

    fn mul(&self, other: RugRat) -> RugRat {
        RugRat(self.0.clone() * other.0.clone())
    }


    fn sqrt(&self) -> RugRat {
        RugRat( Rational::from_f64( self.0.clone().to_f64().sqrt() ).unwrap())
    }

    fn value(&self) -> RugRat {
        self.clone()
    }
    
        fn one() -> RugRat {
        RugRat(Rational::from((1,1)))
    }
    
        fn zero() -> RugRat {
        RugRat(Rational::from((0,1)))
    }
}


struct PointRugRat {
    x: RugRat,
    y: RugRat,
}

impl Point<RugRat> for PointRugRat {
    fn coordinates(&self) -> Vec<RugRat> {
        vec![self.x.clone(), self.y.clone()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_RugRat_add() {
        let a = RugRat(Rational::from((1,1)));
        let b = RugRat(Rational::from((2,1)));
        assert_eq!(a.add(b).0, Rational::from((3,1)));
    }


    #[test]
    fn test_PointRugRat() {
    let point1 = PointRugRat { x: RugRat(Rational::from((0,1))), y: RugRat(Rational::from((0,1))) };
    let point2 = PointRugRat { x: RugRat(Rational::from((4,1))), y: RugRat(Rational::from((3,1))) };

    let q = quadrance(&point1, &point2);
    let d = distance(&point1, &point2);

    println!("Distance: {}", d.0);
    println!("Quadrance: {}", q.0);
        assert_eq( q.0, RugRat(Rational::from(25,1)); 
        assert_eq( d.0, RugRat(Rational::from(5,1)); 
    }
}




