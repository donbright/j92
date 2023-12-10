use crate::PseudoField;
use crate::Point;
use crate::distance;


#[derive(Clone, Copy)]
struct Fieldf32(f32);

impl PseudoField<Fieldf32> for Fieldf32 {
    fn add(&self, other: Fieldf32) -> Fieldf32 {
        Fieldf32(self.0 + other.0)
    }


    fn sub(&self, other: Fieldf32) -> Fieldf32 {
        Fieldf32(self.0 - other.0)
    }

    fn mul(&self, other: Fieldf32) -> Fieldf32 {
        Fieldf32(self.0 * other.0)
    }


    fn sqrt(&self) -> Fieldf32 {
        Fieldf32(self.0.sqrt())
    }

    fn value(&self) -> Fieldf32 {
        *self
    }
    
        fn one() -> Fieldf32 {
        Fieldf32(1.0f32)
    }
    
        fn zero() -> Fieldf32 {
        Fieldf32(0.0f32)
    }
}


struct Pointf32 {
    x: Fieldf32,
    y: Fieldf32,
}

impl Point<Fieldf32> for Pointf32 {
    fn coordinates(&self) -> Vec<Fieldf32> {
        vec![self.x, self.y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_field_f32_add() {
        let a = Fieldf32(1.0);
        let b = Fieldf32(2.0);
        assert_eq!(a.add(b).0, 3.0);
    }


    #[test]
    fn test_Pointf32() {
    let point1 = Pointf32 { x: Fieldf32(1.0), y: Fieldf32(2.0) };
    let point2 = Pointf32 { x: Fieldf32(4.0), y: Fieldf32(6.0) };

    let dist = distance(&point1, &point2);

    println!("Distance: {}", dist.0);
    }
}




