use crate::PseudoField;
use crate::Point;
use crate::distance;
use rug::Float;

#[derive(Clone)]
struct RugFloat(Float);

impl PseudoField<RugFloat> for RugFloat {
    fn add(&self, other: RugFloat) -> RugFloat {
        RugFloat(self.0.clone() + other.0.clone())
    }


    fn sub(&self, other: RugFloat) -> RugFloat {
        RugFloat(self.0.clone() - other.0.clone())
    }

    fn mul(&self, other: RugFloat) -> RugFloat {
        RugFloat(self.0.clone() * other.0.clone())
    }


    fn sqrt(&self) -> RugFloat {
        RugFloat(self.0.clone().sqrt())
    }

    fn value(&self) -> RugFloat {
        self.clone()
    }
    
        fn one() -> RugFloat {
        RugFloat(Float::with_val(53,1.0))
    }
    
        fn zero() -> RugFloat {
        RugFloat(Float::with_val(53,0.0))
    }
}


struct PointRugFloat {
    x: RugFloat,
    y: RugFloat,
}

impl Point<RugFloat> for PointRugFloat {
    fn coordinates(&self) -> Vec<RugFloat> {
        vec![self.x.clone(), self.y.clone()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_RugFloat_add() {
        let a = RugFloat(Float::with_val(53,1.0));
        let b = RugFloat(Float::with_val(53,2.0));
        assert_eq!(a.add(b).0, Float::with_val(53,3.0));
    }


    #[test]
    fn test_PointRugFloat() {
    let point1 = PointRugFloat { x: RugFloat(Float::with_val(53,1.0)), y: RugFloat(Float::with_val(53,2.0)) };
    let point2 = PointRugFloat { x: RugFloat(Float::with_val(53,4.0)), y: RugFloat(Float::with_val(53,6.0)) };

    let dist = distance(&point1, &point2);

    println!("Distance: {}", dist.0);
    }
}




