use crate::distance;
use crate::PseudoField;
use crate::Point;
use crate::Edge;
use crate::Face;

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy,Debug)]
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

#[derive(Debug)]
struct EdgeF64<T> {
    start: T,
    end: T,
}

impl<X, T, U> Edge<X, T, U> for EdgeF64<X> 
where
    X: Point<T, U> + 'static + Copy,
    U: PseudoField<T>,
 {
    fn points(&self) -> Box<dyn Iterator<Item = X>> {
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
    println!("{:?}",distance(&e1.start, &e1.end));
}


#[derive(Debug)]
struct FaceF64<T> {
    edges: Vec<T>,
}

impl<Z, X, T, U> Face<Z, X, T, U> for FaceF64<Z> 
where
    Z: Edge<X,T,U> + 'static + Copy,
    X: Point<T, U> + 'static + Copy,
    U: PseudoField<T>,
 {
    fn edges(&self) -> Box<dyn Iterator<Item = Z>> {
	Box::new(self.edges.clone().into_iter())
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
    let point3 = PointF64 {
        x: FieldF64(0.0),
        y: FieldF64(0.0),
    };

    let e1 = EdgeF64 {
        start: point1,
        end: point2,
    };
    let e2 = EdgeF64 {
        start: point2,
        end: point3,
    };
    let e3 = EdgeF64 {
        start: point3,
        end: point1,
    };
    let f = FaceF64 { edges: vec![e1, e2, e3]} ;
    println!("{:?}",f);
}



#[cfg(test)]
mod tests {
    use super::*;
}
