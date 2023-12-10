use crate::PseudoField;
use crate::Point;
use crate::distance;
use half::f16;

#[derive(Clone, Copy)]
struct Fieldf16(f16);

impl PseudoField<Fieldf16> for Fieldf16 {
    fn add(&self, other: Fieldf16) -> Fieldf16 {
        Fieldf16(self.0 + other.0)
    }


    fn sub(&self, other: Fieldf16) -> Fieldf16 {
        Fieldf16(self.0 - other.0)
    }

    fn mul(&self, other: Fieldf16) -> Fieldf16 {
        Fieldf16(self.0 * other.0)
    }


    fn sqrt(&self) -> Fieldf16 {
        Fieldf16( f16::from_f64(self.0.to_f64().sqrt()) )
    }

    fn value(&self) -> Fieldf16 {
        *self
    }
    
        fn one() -> Fieldf16 {
        Fieldf16(f16::from_f64(1.0))
    }
    
        fn zero() -> Fieldf16 {
        Fieldf16(f16::from_f64(0.0))
    }
}


struct Pointf16 {
    x: Fieldf16,
    y: Fieldf16,
}

impl Point<Fieldf16> for Pointf16 {
    fn coordinates(&self) -> Vec<Fieldf16> {
        vec![self.x, self.y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_field_f16_add() {
        let a = Fieldf16(f16::from_f64(1.0));
        let b = Fieldf16(f16::from_f64(2.0));
        assert_eq!(a.add(b).0, f16::from_f64(3.0));
    }


    #[test]
    fn test_Pointf16() {
    let point1 = Pointf16 { x: Fieldf16(f16::from_f64(1.0)), y: Fieldf16(f16::from_f64(2.0)) };
    let point2 = Pointf16 { x: Fieldf16(f16::from_f64(4.0)), y: Fieldf16(f16::from_f64(6.0)) };

    let dist = distance(&point1, &point2);

    println!("Distance: {}", dist.0);
    }
}




