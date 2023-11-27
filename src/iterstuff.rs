use crate::Point;
use crate::seed_points;

use num_traits::Float;
use std::iter::Iterator;
use std::str::FromStr;
use std::fmt::Debug;

// support f32 and f64
use num_traits;
// and arbitrary precision
use rug;
use rug::ops::CompleteRound;
use rug::Complete;

/// Implement new Points for various number types
impl<T: FromStr> Point<T> {
    pub fn new_from_str(x: &str, y: &str, z: &str) -> Self
    where
        T: std::fmt::Display,
        <T as FromStr>::Err: Debug,
    {
        Point {
            x: T::from_str(x).unwrap(),
            y: T::from_str(y).unwrap(),
            z: T::from_str(z).unwrap(),
        }
    }
}
impl Point<rug::Float> {
    pub fn new_rugfloat_from_str(x: &str, y: &str, z: &str, precision: &u32) -> Self
    {
        Point {
            x: rug::Float::parse(x).unwrap().complete(*precision),
            y: rug::Float::parse(y).unwrap().complete(*precision),
            z: rug::Float::parse(z).unwrap().complete(*precision),
        }
    }
}
impl Point<rug::Rational> {
    pub fn new_rugrat_from_str(x: &str, y: &str, z: &str) -> Self
    {
        Point {
            x: rug::Rational::parse(x).unwrap().complete(),
            y: rug::Rational::parse(y).unwrap().complete(),
            z: rug::Rational::parse(z).unwrap().complete(),
        }
    }
}


/// Implement iterator adaptors to turn items into floating point
pub trait ToFloat<'a, T>
where
    T: num_traits::Float,
{
    fn to_ntfloat(self) -> Box<dyn Iterator<Item = Point<T>> + 'a>;
}

impl<'a, I, T> ToFloat<'a, T> for I
where
    I: Iterator<Item = Point<String>> + 'a,
    T:num_traits::Float + FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    fn to_ntfloat(self) -> Box<dyn Iterator<Item = Point<T>> + 'a> {
        Box::new(self.map(|point| {
            Point::new(
                T::from_str(&point.x).unwrap(),
                T::from_str(&point.y).unwrap(),
                T::from_str(&point.z).unwrap(),
            )
        }))
    }
}

#[cfg(test)]
#[test]
fn test_ntfloat_adapter() {
    itertools::assert_equal(
        seed_points("±2,0,±1").to_ntfloat(),
        vec![
            Point::<f32>::new(2., 0., 1.),
            Point::<f32>::new(2., 0., -1.),
            Point::<f32>::new(-2., 0., 1.),
            Point::<f32>::new(-2., 0., -1.),
        ],
    );

    itertools::assert_equal(
        seed_points("±2,0,±1").to_ntfloat(),
        vec![
            Point::<f64>::new(2., 0., 1.),
            Point::<f64>::new(2., 0., -1.),
            Point::<f64>::new(-2., 0., 1.),
            Point::<f64>::new(-2., 0., -1.),
        ],
    );
    
        // f16 doesn't allow floating point literals but we can build from string
    itertools::assert_equal(
        seed_points("±2,0,±1").to_ntfloat(),
        vec![
            Point::<half::f16>::new_from_str("2", "0", "1"),
            Point::<half::f16>::new_from_str("2", "0", "-1"),
            Point::<half::f16>::new_from_str("-2", "0", "1"),
            Point::<half::f16>::new_from_str("-2", "0", "-1"),
        ],
    );


}


pub trait ToRugFloat<'a>
{
    fn to_rugfloat(self,precision:u32) -> Box<dyn Iterator<Item = Point<rug::Float>> + 'a>;
}

impl<'a, I> ToRugFloat<'a> for I
where
    I: Iterator<Item = Point<String>> + 'a,
{
    fn to_rugfloat(self,precision:u32) -> Box<dyn Iterator<Item = Point<rug::Float>> + 'a> {
        Box::new(self.map(move |point| {
            Point::<rug::Float>::new(
                rug::Float::parse(&point.x).unwrap().complete(precision),
                rug::Float::parse(&point.y).unwrap().complete(precision),
                rug::Float::parse(&point.z).unwrap().complete(precision),
            )
        }))
    }
}


#[cfg(test)]
#[test]
fn test_rugfloat_adapter() {

    // rug Float allows arbitrary precision floating point
    let precision = 256;
    itertools::assert_equal(
        seed_points("±2,0,±1").to_rugfloat(precision),
        vec![
            Point::new_rugfloat_from_str("2", "0", "1", &precision),
            Point::new_rugfloat_from_str("2", "0", "-1", &precision),
            Point::new_rugfloat_from_str("-2", "0", "1", &precision),
            Point::new_rugfloat_from_str("-2", "0", "-1", &precision),
        ],
    );
    
}

pub trait ToRugRational<'a>
{
    fn to_rugrat(self) -> Box<dyn Iterator<Item = Point<rug::Rational>> + 'a>;
}

impl<'a, I> ToRugRational<'a> for I
where
    I: Iterator<Item = Point<String>> + 'a,
{
    fn to_rugrat(self) -> Box<dyn Iterator<Item = Point<rug::Rational>> + 'a> {
        Box::new(self.map(move |point| {
            Point::<rug::Rational>::new(
                rug::Rational::parse(&point.x).unwrap().complete(),
                rug::Rational::parse(&point.y).unwrap().complete(),
                rug::Rational::parse(&point.z).unwrap().complete(),
            )
        }))
    }
}


#[cfg(test)]
#[test]
fn test_rugrat_adapter() {

    // rug Rational allows arbitrary precision Rational numbers
    itertools::assert_equal(
        seed_points("±2,0,±1").to_rugrat(),
        vec![
            Point::new_rugrat_from_str("2", "0", "1"),
            Point::new_rugrat_from_str("2", "0", "-1"),
            Point::new_rugrat_from_str("-2", "0", "1"),
            Point::new_rugrat_from_str("-2", "0", "-1"),
        ],
    );
    
}


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
