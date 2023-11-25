use crate::Point;
use num_traits::Float;
use std::iter::Iterator;

/// Implement the Rotations iterator adapter
struct Rotations<'a, I, T>
where
    I: Iterator<Item = &'a Point<T>>,
    T: 'a + Float,
{
    iter: I,
    last_point: Option<Point<T>>,
    start_count: i8,
    count: i8,
}

impl<'a, I, T> Iterator for Rotations<'a, I, T>
where
    I: Iterator<Item = &'a Point<T>>,
    T: 'a + Float + std::fmt::Debug,
{
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.count = self.count - 1;
        if self.count == self.start_count - 1 {
            println!("{:?}", self.last_point);
            self.last_point = self.iter.next().copied();
        } else {
            let p = self.last_point.unwrap();
            self.last_point = Some(Point::new(p.z, p.x, p.y));
        };
        if self.count == 0 {
            self.count = self.start_count;
        }
        self.last_point
    }
}

/// Implement the Rotations method for any iterator over &Point<T>
trait RotationsExt<'a, T>: Iterator<Item = &'a Point<T>> + Sized
where
    T: 'a + Float,
{
    fn rotations(self, count: i8) -> Rotations<'a, Self, T> {
        Rotations {
            iter: self,
            start_count: count,
            count: count,
            last_point: None,
        }
    }
}

impl<'a, I, T> RotationsExt<'a, T> for I
where
    I: Iterator<Item = &'a Point<T>>,
    T: 'a + Float,
{
}

#[cfg(test)]
#[test]
fn test_rotations() {
    itertools::assert_equal(
        vec![Point::new(1., 2., 3.)].iter().rotations(3),
        vec![
            Point::new(1., 2., 3.),
            Point::new(3., 1., 2.),
            Point::new(2., 3., 1.),
        ]
        .into_iter(),
    );
    itertools::assert_equal(
        vec![Point::new(1., 2., 3.)].iter().rotations(2),
        vec![Point::new(1., 2., 3.), Point::new(3., 1., 2.)].into_iter(),
    );
    itertools::assert_equal(
        vec![Point::new(1., 2., 3.), Point::new(1., 2., -3.)]
            .iter()
            .rotations(3),
        vec![
            Point::new(1., 2., 3.),
            Point::new(3., 1., 2.),
            Point::new(2., 3., 1.),
            Point::new(1., 2., -3.),
            Point::new(-3., 1., 2.),
            Point::new(2., -3., 1.),
        ]
        .into_iter(),
    );
}
