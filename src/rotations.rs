use num_traits::Float;
use std::iter::Iterator;

// Define the Point struct
#[derive(Clone, Copy, Debug)]
struct Point<T> where T: Float {
    x: T,
    y: T,
}

// Implement the Blah iterator adapter
struct Blah<'a, I, T>
where
    I: Iterator<Item = &'a Point<T>>,
    T: 'a + Float,
{
    iter: I,
    next_item: Option<Point<T>>,
    flip: bool,
}

impl<'a, I, T> Iterator for Blah<'a, I, T>
where
    I: Iterator<Item = &'a Point<T>>,
    T: 'a + Float,
{
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.next_item.take() {
            if self.flip {
                self.flip = false;
                return Some(Point { x: item.y, y: item.x });
            } else {
                return Some(item);
            }
        }

        if let Some(item) = self.iter.next() {
            self.next_item = Some(*item);
            self.flip = true;
            return Some(*item);
        }

        None
    }
}

// Implement the blah method for any iterator over &Point<T>
trait BlahExt<'a, T>: Iterator<Item = &'a Point<T>> + Sized
where
    T: 'a + Float,
{
    fn blah(self) -> Blah<'a, Self, T> {
        Blah {
            iter: self,
            next_item: None,
            flip: false,
        }
    }
}

impl<'a, I, T> BlahExt<'a, T> for I 
where 
    I: Iterator<Item = &'a Point<T>>, 
    T: 'a + Float 
{}

#[cfg(test)]
#[test]
test_rotations() {
    // Example usage
    let points = vec![
        Point { x: 1.0, y: 2.0 },
        Point { x: 3.0, y: 4.0 },
    ];

    let processed_points: Vec<_> = points.iter().blah().collect();
    println!("{:?}", processed_points);
}

