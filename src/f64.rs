use crate::PseudoField;
use crate::Point;
use crate::distance;
use crate::*;

#[derive(Clone, Copy)]
struct FieldF64(f64);

impl PseudoField<FieldF64> for FieldF64 {
    fn add(&self, other: FieldF64) -> FieldF64 {
        FieldF64(self.0 + other.0)
    }


    fn sub(&self, other: FieldF64) -> FieldF64 {
        FieldF64(self.0 - other.0)
    }

    fn mul(&self, other: FieldF64) -> FieldF64 {
        FieldF64(self.0 * other.0)
    }


    fn sqrt(&self) -> FieldF64 {
        FieldF64(self.0.sqrt())
    }

    fn value(&self) -> FieldF64 {
        *self
    }
    
        fn one() -> FieldF64 {
        FieldF64(1.0f64)
    }
    
        fn zero() -> FieldF64 {
        FieldF64(0.0f64)
    }
}

#[derive(Clone, Copy)]
struct PointF64 {
    x: FieldF64,
    y: FieldF64,
}

#[derive(Clone, Copy)]
struct EdgeF64 {
	p1: PointF64,
	p2: PointF64,
}

impl Point<FieldF64> for PointF64 {

    fn coordinates(&self) -> Box<dyn Iterator<Item = FieldF64>> {
        Box::new(vec![self.x.clone(), self.y.clone()].into_iter())
    }
}

    #[test]
    fn test_PointF64() {
    let point1 = PointF64 { x: FieldF64(1.0), y: FieldF64(2.0) };
    let point2 = PointF64 { x: FieldF64(4.0), y: FieldF64(6.0) };

    let dist = distance(&point1, &point2);

    println!("Distance: {}", dist.0);
    }



impl Edge<PointF64,FieldF64> for EdgeF64 {

    fn points(&self) -> Box<dyn Iterator<Item = PointF64>> {
        Box::new(vec![self.p1, self.p2].into_iter())
    }
}

#[test]
    fn test_EdgeF64() {
    let point1 = PointF64 { x: FieldF64(1.0), y: FieldF64(2.0) };
    let point2 = PointF64 { x: FieldF64(4.0), y: FieldF64(6.0) };
    let e1 = EdgeF64 { p1:point1,p2:point2 };

    }
    
#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_field_f64_add() {
        let a = FieldF64(1.0);
        let b = FieldF64(2.0);
        assert_eq!(a.add(b).0, 3.0);
    }



    

}




