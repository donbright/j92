use crate::distance;
use crate::PseudoField;
use crate::Point;
use crate::Edge;

#[derive(Clone, Copy)]
struct FieldF64(f64);

impl PseudoField<f64> for FieldF64 {
    fn add(&self, other: &Self) -> Self {
        FieldF64(self.0 + other.0)
    }

    fn sub(&self, other: &Self) -> Self {
        FieldF64(self.0 - other.0)
    }

    fn mul(&self, other:&Self) -> Self {
        FieldF64(self.0 * other.0)
    }

    fn sqrt(&self) -> Self {
        FieldF64(self.0.sqrt())
    }

    fn zero() -> Self {
        FieldF64(0.0)
    }
    
        fn one() -> Self {
        FieldF64(1.0)
    }
}


#[test]
fn test_field_f64_add() {
    let a = FieldF64(1.0);
    let b = FieldF64(2.0);
    assert_eq!(a.add(&b).0, 3.0);
}

#[derive(Clone, Copy)]
struct PointF64 {
    x: FieldF64,
    y: FieldF64,
}

impl Point<f64, FieldF64> for PointF64 {
    fn coordinates(&self) -> Box<dyn Iterator<Item = FieldF64>> {
        Box::new(vec![self.x, self.y].into_iter())
    }
}


#[test]
fn test_PointF64() {
    let point1 = PointF64 {
        x: FieldF64(1.0),
        y: FieldF64(2.0),
    };
    let point2 = PointF64 {
        x: FieldF64(4.0),
        y: FieldF64(6.0),
    };

    let dist = distance(&point1, &point2);

    println!("Distance: {}", dist.0);
}

struct EdgeF64<T> {
    start: T,
    end: T,
}

impl<T, U, X> Edge<T, U, X> for EdgeF64<T> 
where
    T: Point<U, X> + 'static + Copy,
    X: PseudoField<U>,
 {
    fn points(&self) -> Box<dyn Iterator<Item = T>> {
        Box::new(vec![self.start, self.end].into_iter())
    }
}



#[test]
fn test_EdgeF64() {
    let point1 = PointF64 {
        x: FieldF64(1.0),
        y: FieldF64(2.0),
    };
    let point2 = PointF64 {
        x: FieldF64(4.0),
        y: FieldF64(6.0),
    };
    let e1 = EdgeF64 {
        start: point1,
        end: point2,
    };
}


/*
#[derive(Clone)]
struct FaceF64 {
    edges: Vec<EdgeF64>,
}

impl Face<EdgeF64, PointF64, FieldF64> for FaceF64 {
    fn edges(&self) -> Box<dyn Iterator<Item = EdgeF64>> {
        self.edges.into_iter()
    }
}

#[test]
fn test_FaceF64() {
    let point1 = PointF64 {
        x: FieldF64(1.0),
        y: FieldF64(2.0),
    };
    let point2 = PointF64 {
        x: FieldF64(4.0),
        y: FieldF64(6.0),
    };
    let e1 = EdgeF64 {
        p1: point1,
        p2: point2,
    };
}
*/

#[cfg(test)]
mod tests {
    use super::*;
}
