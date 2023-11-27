pub mod iterstuff;

pub enum JohnsonSolid {
    SsquarePyramid,
    PentagonalPyramid,
    TriangularCupola,
    SquareCupola,
    PentagonalCupola,
    PentagonalRotunda,
    ElongatedTriangularPyramid,
    ElongatedSquarePyramid,
    ElongatedPentagonalPyramid,
    GyroelongatedSquarePyramid,
    GyroelongatedPentagonalPyramid,
    TriangularDipyramid,
    PentagonalDipyramid,
    ElongatedTriangularDipyramid,
    ElongatedSquareDipyramid,
    ElongatedPentagonalDipyramida,
    GyroelongatedSquareDipyramid,
    ElongatedTriangularCupola,
    ElongatedSquareCupola,
    ElongatedPentagonalCupola,
    ElongatedPentagonalRotunda,
    GyroelongatedTriangularCupola,
    GyroelongatedSquareCupola,
    GyroelongatedPentagonalCupola,
    GyroelongatedPentagonalRotunda,
    Gyrobifastigium,
    TriangularOrthobicupola,
    SquareOrthobicupola,
    SquareGyrobicupola,
    PentagonalOrthobicupola,
    PentagonalGyrobicupola,
    PentagonalOrthocupolarotunda,
    PentagonalGyrocupolarotunda,
    PentagonalOrthobirotunda,
    ElongatedTriangularOrthobicupola,
    ElongatedTriangularGyrobicupola,
    ElongatedSquareGyrobicupola,
    ElongatedPentagonalOrthobicupola,
    ElongatedPentagonalGyrobicupola,
    ElongatedPentagonalOrthocupolarotunda,
    ElongatedPentagonalGyrocupolarotunda,
    ElongatedPentagonalOrthobirotunda,
    ElongatedPentagonalGyrobirotunda,
    GyroelongatedTriangularBicupola,
    GyroelongatedSquareBicupola,
    GyroelongatedPentagonalBicupola,
    GyroelongatedPentag,
    GyroelongatedPentagonalBirotunda,
    AugmentedTriangularPrism,
    BiaugmentedTriangularPrism,
    TriaugmentedTriangularPrism,
    AugmentedPentagonalPrism,
    BiaugmentedPentagonalPrism,
    AugmentedHexagonalPrism,
    ParabiaugmentedHexagonalPrism,
    MetabiaugmentedHexagonalPrism,
    TriaugmentedHexagonalPrism,
    AugmentedDodecahedron,
    ParabiaugmentedDodecahedron,
    MetabiaugmentedDodecahedron,
    TriaugmentedDodecahedron,
    MetabidiminishedIcosahedron,
    TridiminishedIcosahedron,
    AugmentedTridiminishedIcosahedron,
    AugmentedTruncatedTetrahedron,
    AugmentedTruncatedCube,
    BiaugmentedTruncatedCube,
    AugmentedTruncatedDodecahedron,
    ParabiaugmentedTruncated,
    MetabiaugmentedTruncated,
    TriaugmentedTruncatedDodecahedron,
    GyrateRhombicosidodecahedron,
    ParabigyrateRhombicosidodecahedron,
    MetabigyrateRhombicosidodecahedron,
    TrigyrateRhombicosidodecahedron,
    DiminishedRhombicosidodecahedron,
    ParagyrateDiminished,
    MetagyrateDiminished,
    BigyrateDiminished,
    ParabidiminishedRhombicosidodecahedron,
    MetabidiminishedRhombicosidodecahedron,
    GyrateBidiminished,
    TridiminishedRhombicosidodecahedron,
    SnubDisphenoid,
    SnubSquareAntiprism,
    Sphenocorona,
    AugmentedSphenocorona,
    Sphenomegacorona,
    Hebesphenomegacorona,
    Disphenocingulum,
    Bilunabirotunda,
    TriangularHebesphenorotunda,
}

// lets us convert strings to numbers
use std::fmt::Debug;
use std::str::FromStr;

// to help us deal with the +/- notation
use itertools::iproduct;

// support f32 and f64
use num_traits;

use crate::iterstuff::*;

#[derive(Clone, Copy, Debug)]
pub struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point { x, y, z }
    }
}

struct Edge<T> {
    a: Point<T>,
    b: Point<T>,
}

/// Equation of 3 dimensional plane, in form ax + by + c = 0
struct Plane<T> {
    a: T,
    b: T,
    c: T,
}


impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, o: &Self) -> bool {
        self.x == o.x && self.y == o.y && self.z == o.z
    }
}

/// take a string "x" and return x a floating point number
/// special symbols understood:
///    Φ golden ratio
///	   Φ′ golden ratio conjugate
///    Φ⁻¹ inverse golden ratio
///    √2 square root of 2
///    etc etc
pub fn floatify<T>(s: &str) -> T
where
    T: num_traits::Float + FromStr + std::fmt::Debug + std::fmt::Display,
    <T as FromStr>::Err: Debug,
{
    let one: T = T::one();
    let two: T = one + one;
    let three: T = two + one;
    let four: T = two + two;
    let five: T = four + one;

    let phi = (one + five.sqrt()) / two;
    let phiprime = (one - five.sqrt()) / two;
    println!("string input {}", s);
    let s = s.replace("φ", "Φ");
    let f = s
        .replace("√(Φ′+2)", &format!("{}", (phiprime + two).sqrt()))
        .replace("√(Φ+2)", &format!("{}", (phi + two).sqrt()))
        .replace("(Φ+2)", &format!("{}", phi + two))
        .replace("Φ-1", &format!("{}", phi - one))
        .replace("Φ²/2", &format!("{}", phi * phi / two))
        .replace("Φ²", &format!("{}", phi * phi))
        .replace("Φ³", &format!("{}", phi * phi * phi))
        .replace("Φ⁻¹", &format!("{}", one / phi))
        .replace("Φ/2", &format!("{}", phi / two))
        .replace("Φ′", &format!("{}", phiprime))
        .replace("(1+√2)", &format!("{}", one + two.sqrt()))
        .replace("(2+√2)", &format!("{}", two + two.sqrt()))
        .replace("1+√2", &format!("{}", one + two.sqrt()))
        .replace("√3", &format!("{}", three.sqrt()))
        .replace("√2", &format!("{}", two.sqrt()))
        .replace("2Φ", &format!("{}", two * phi))
        .replace("Φ", &format!("{}", phi))
        .replace("--", "")
        .replace("_", "") // mimic rust float literal syntax
        .replace(" ", "");
    println!("float output {:?}", f);
    s.parse::<T>().unwrap()
}

/// given a &str like = "±2,0,0" expand the plusminus and create
/// an Iterator of new points for each variation, like [+2,0,0][-2,2,0]
///
/// for "±2,0,±1" generate [-2,0,1][2,0,1][-2,0,-1][2,0,-1]
///
/// This notation is often used in geometry text, not sure of the name.
/// But it is very nice and compact especially if combined with other
/// iterator adapters such as chain() or rotations()
///
/// example:
/// ```
/// assert!( j92::seed_points("±2,0,±1").collect::<Vec<_>>()==
///        vec![j92::Point::new_from_str("2", "0", "1"),
///             j92::Point::new_from_str("2", "0", "-1"),
///             j92::Point::new_from_str("-2", "0", "1"),
///             j92::Point::new_from_str("-2", "0", "-1")] );
/// ```
///
pub fn seed_points(s: &str) -> impl Iterator<Item = Point<String>> {
    let mut v: Vec<Vec<String>> = Vec::new();
    for n in s.replace(" ", "").split(",") {
        match n.chars().nth(0).unwrap() {
            '±' => {
                let f: String = n.split("±").nth(1).unwrap().to_string();
                v.push(vec![f.clone(), format!("-{}", f)]);
            }
            _ => v.push(vec![n.to_string()]),
        }
    }
    iproduct!(v[0].clone(), v[1].clone(), v[2].clone()).map(|v| Point::new(v.0, v.1, v.2))
}

#[cfg(feature = "not_used")]
mod unused {

    // use CHull to generate a convex hull of the input points.
    // this can be used to help build the Johnson Solids.
    // yes, this is extremely slow, we have to transfer the CHull data
    // into Polyhedron-Ops data.
    fn convex_hull(p: &Points) -> Faces {
        let mut v = vec![];
        for q in p {
            v.push(vec![q.x, q.y, q.z]);
        }
        let hull = ConvexHullWrapper::try_new(&v, None).unwrap();
        let m = (hull.vertices_indices().1)
            .into_iter()
            .collect::<Vec<usize>>();
        let m2: Vec<u32> = m.into_iter().map(|x| x as u32).collect();
        m2.chunks(3).map(|chunk| chunk.to_vec()).collect()
    }

    impl Polyhedron {
        pub fn tetrahedron() -> Self {
            Self {
                positions: seed_points("1,1,1")
                    .chain(seed_points("1,-1,-1").rotations(3))
                    .collect(),
                face_index: vec![vec![2, 1, 0], vec![3, 2, 0], vec![1, 3, 0], vec![2, 3, 1]],
                face_set_index: vec![(0..4).collect()],
                name: String::from("T"),
            }
        }

        pub fn hexahedron() -> Self {
            let pts = seed_points("±1,±1,±1").collect();
            let hull = convex_hull(&pts);
            Self {
                positions: pts,
                face_index: hull,
                face_set_index: vec![(0..6).collect()],
                name: String::from("C"),
            }
        }

        #[inline]
        /// Alias for [`hexahedron()`](Self::hexahedron()).
        pub fn cube() -> Self {
            Self::hexahedron()
        }

        pub fn octahedron() -> Self {
            let pts = seed_points("0,0,±1").rotations(3).collect();
            let hull = convex_hull(&pts);
            Self {
                positions: pts,
                face_index: hull,
                face_set_index: vec![(0..8).collect()],
                name: String::from("O"),
            }
        }

        pub fn dodecahedron() -> Self {
            let pts = seed_points("0,±Φ²,±1")
                .rotations(3)
                .chain(seed_points("±Φ,±Φ,±Φ"))
                .collect();
            let hull = convex_hull(&pts);
            Self {
                positions: pts,
                face_index: hull,
                face_set_index: vec![(0..12).collect()],
                name: String::from("D"),
            }
        }

        pub fn icosahedron() -> Self {
            Self {
                positions: seed_points("±1,0,±Φ").rotations(3).collect(),
                face_index: vec![
                    vec![6, 5, 0],
                    vec![10, 5, 6],
                    vec![1, 8, 2],
                    vec![1, 3, 8],
                    vec![4, 8, 9],
                    vec![9, 10, 4],
                    vec![10, 9, 11],
                    vec![10, 11, 5],
                    vec![11, 7, 5],
                    vec![7, 0, 5],
                    vec![1, 7, 3],
                    vec![1, 0, 7],
                    vec![1, 2, 0],
                    vec![0, 2, 6],
                    vec![6, 2, 4],
                    vec![2, 8, 4],
                    vec![10, 6, 4],
                    vec![3, 7, 11],
                    vec![9, 3, 11],
                    vec![9, 8, 3],
                ],
                face_set_index: vec![(0..20).collect()],
                name: String::from("I"),
            }
        }

        /// common code for prism and antiprism
        #[inline]
        fn protoprism(n: Option<usize>, anti: bool) -> Self {
            let n = n.unwrap_or(3);

            // Angles.
            let theta = f32::TAU() / n as f32;
            let twist = if anti { theta / 2.0 } else { 0.0 };
            // Half-edge.
            let h = (theta * 0.5).sin();

            let mut face_index = vec![
                (0..n).map(|i| i as VertexKey).collect::<Vec<_>>(),
                (n..2 * n).rev().map(|i| i as VertexKey).collect::<Vec<_>>(),
            ];

            // Sides.
            if anti {
                face_index.extend(
                    (0..n)
                        .map(|i| {
                            vec![
                                i as VertexKey,
                                (i + n) as VertexKey,
                                ((i + 1) % n) as VertexKey,
                            ]
                        })
                        .chain((0..n).map(|i| {
                            vec![
                                (i + n) as VertexKey,
                                ((i + 1) % n + n) as VertexKey,
                                ((i + 1) % n) as VertexKey,
                            ]
                        })),
                );
            } else {
                face_index.extend((0..n).map(|i| {
                    vec![
                        i as VertexKey,
                        (i + n) as VertexKey,
                        ((i + 1) % n + n) as VertexKey,
                        ((i + 1) % n) as VertexKey,
                    ]
                }));
            };

            Self {
                name: format!("{}{}", if anti { "A" } else { "P" }, n),
                positions: (0..n)
                    .map(move |i| {
                        Point::new(
                            (i as f32 * theta).cos() as _,
                            h,
                            (i as f32 * theta).sin() as _,
                        )
                    })
                    .chain((0..n).map(move |i| {
                        Point::new(
                            (twist + i as f32 * theta).cos() as _,
                            -h,
                            (twist + i as f32 * theta).sin() as _,
                        )
                    }))
                    .collect(),

                face_index,
                face_set_index: Vec::new(),
            }
        }

        pub fn prism(n: Option<usize>) -> Self {
            Self::protoprism(n, false)
        }

        pub fn antiprism(n: Option<usize>) -> Self {
            Self::protoprism(n, true)
        }

        pub fn pyramid(n: Option<usize>) -> Self {
            let n = n.unwrap_or(4);
            let c0 = 1.0;
            let height = c0;

            // Angle.
            let theta = f32::TAU() / n as f32;

            // bottom face
            let mut face_index = vec![(0..n).rev().map(|i| i as VertexKey).collect::<Vec<_>>()];

            // Sides.
            face_index.extend((0..n).map(|i| {
                vec![
                    (n) as VertexKey,
                    (i) as VertexKey,
                    ((i + 1) % n) as VertexKey,
                ]
            }));

            Self {
                name: format!("Y{}", n),
                positions: (0..n)
                    .map(move |i| {
                        Point::new(
                            (i as f32 * theta).cos() as _,
                            -c0 / 2.0,
                            (i as f32 * theta).sin() as _,
                        )
                    })
                    .chain(once(Point::new(0.0, -c0 / 2.0 + height, 0.0)))
                    .collect(),

                face_index,
                face_set_index: Vec::new(),
            }
        }

        pub fn triangular_cupola() -> Self {
            Self {
                positions: seed_points("±2,0,0")
                    .chain(seed_points("±1,±√3,0"))
                    .chain(seed_points("±1,√3,√3"))
                    .chain(seed_points("1,-2/√3,√3"))
                    .collect(),
                face_index: vec![vec![2, 1, 0], vec![3, 2, 0], vec![1, 3, 0], vec![2, 3, 1]],
                face_set_index: vec![(0..4).collect()],
                name: String::from("T"),
            }
        }

        pub fn square_cupola() -> Self {
            Self {
                positions: seed_points("±1,±1+√2,0")
                    .chain(seed_points("±1+√2,±1,0"))
                    .chain(seed_points("±1,±1,√2"))
                    .collect(),
                face_index: vec![
                    vec![2, 6, 7, 3, 1, 5, 4, 0],
                    vec![10, 8, 9, 11],
                    vec![1, 3, 11, 9],
                    vec![2, 0, 8, 10],
                    vec![4, 5, 9, 8],
                    vec![6, 10, 11, 7],
                    vec![0, 4, 8],
                    vec![5, 1, 9],
                    vec![3, 7, 11],
                    vec![6, 2, 10],
                ],
                face_set_index: vec![(0..4).collect()],
                name: String::from("J4"),
            }
        }

        // https://en.wikipedia.org/wiki/Rhombicosidodecahedron
        pub fn truncated_cuboctahedron() -> Self {
            let points = seed_points("±1,±(1+√2),±(2+√2)")
                .rotations(3)
                .chain(seed_points("±(1+√2),±1,±(2+√2)").rotations(3))
                .chain(seed_points("±(2+√2),±1,±(1+√2)").rotations(3))
                .collect();
            let hull = convex_hull(&points);
            Self {
                positions: points,
                face_index: hull,
                face_set_index: vec![(0..1).collect()],
                name: String::from("bC"),
            }
        }

        // https://en.wikipedia.org/wiki/Rhombicosidodecahedron
        pub fn rhombicosidodecahedron() -> Self {
            let points = seed_points("±1, ±1, ±φ³")
                .rotations(3)
                .chain(seed_points("±φ², ±φ, ±2φ").rotations(3))
                .chain(seed_points("±(φ+2), 0, ±φ²").rotations(3))
                .collect();
            let hull = convex_hull(&points);
            Self {
                positions: points,
                face_index: hull,
                face_set_index: vec![(0..1).collect()],
                name: String::from("eD"),
            }
        }

        // https://en.wikipedia.org/wiki/icosidodecahedron
        pub fn icosidodecahedron() -> Self {
            let points = seed_points("0, 0, ±2φ")
                .rotations(3)
                .chain(seed_points("±1, ±φ, ±φ²").rotations(3))
                .collect();
            let hull = convex_hull(&points);
            Self {
                positions: points,
                face_index: hull,
                face_set_index: vec![(0..1).collect()],
                name: String::from("aD"),
            }
        }

        // https://en.wikipedia.org/wiki/snub_cube
        pub fn snub_cube() -> Self {
            // the filters make the first group of points have an even number
            // of negatives, the second group have an odd number of negatives.
            let points = seed_points("±1,±0.54368,±0.2956")
                .rotations(3)
                .filter(|p| p.x * p.y * p.z > 0.)
                .chain(
                    seed_points("±0.54368,±1,±0.2956")
                        .rotations(3)
                        .filter(|p| p.x * p.y * p.z < 0.),
                )
                .collect();
            let hull = convex_hull(&points);
            Self {
                positions: points,
                face_index: hull,
                face_set_index: vec![(0..1).collect()],
                name: String::from("aD"),
            }
        }

        // https://en.wikipedia.org/wiki/Pentagonal_cupola
        pub fn pentagonal_cupola() -> Self {
            let points = Self::rhombicosidodecahedron()
                .positions()
                .iter()
                .cloned()
                .filter(|p| p.z + 3.6 < 1.6 * p.y)
                .collect::<Points>();
            let hull = convex_hull(&points);
            Self {
                positions: points,
                face_index: hull,
                face_set_index: vec![(0..1).collect()],
                name: String::from("J5"),
            }
        }

        // https://mathworld.wolfram.com/PentagonalOrthobicupola.html
        pub fn pentagonal_orthobicupola() -> Self {
            let points = Self::pentagonal_cupola()
                .positions()
                .iter()
                .cloned()
                .collect::<Points>();

            let hull = convex_hull(&points);
            Self {
                positions: points,
                face_index: hull,
                face_set_index: vec![(0..1).collect()],
                name: String::from("J5"),
            }
        }

        // https://en.wikipedia.org/wiki/Pentagonal_cupola
        // https://www.geogebra.org/calculator/ef2uskfu
        //https://mathworld.wolfram.com/PentagonalRotunda.html

        pub fn pentagonal_rotunda() -> Self {
            let points = Self::icosidodecahedron()
                .positions()
                .iter()
                .cloned()
                .filter(|p| p.z + 1.6 > p.y)
                .collect::<Points>();
            let hull = convex_hull(&points);
            Self {
                positions: points,
                face_index: hull,
                face_set_index: vec![(0..1).collect()],
                name: String::from("J6"),
            }
        }

        // create a Johnson Solid
        // if n=unimplemented, this creates a Tetrahedron
        pub fn johnson(n: Option<usize>) -> Self {
            match n.unwrap_or(1) {
                1..=2 => Polyhedron::pyramid(Some(n.unwrap_or(1) + 3)),
                3 => Polyhedron::triangular_cupola(),
                4 => Polyhedron::square_cupola(),
                5 => Polyhedron::pentagonal_cupola(),
                6 => Polyhedron::pentagonal_rotunda(),
                _ => Polyhedron::tetrahedron(),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn dumpobj(_p: &Polyhedron) {
            #[cfg(feature = "obj")]
            _p.write_obj(&std::path::PathBuf::from("."), false).unwrap();
        }

        // the Variance of the squared-edges of the given Polyhedron
        // useful for polyhedrons where all edges are equal
        fn edge_variance(p: &Polyhedron) -> f32 {
            let pts = p.positions();
            let eds = p.to_edges();
            let count = eds.clone().len();
            let quads = eds
                .iter()
                .map(|e| pts[e[0] as usize] - pts[e[1] as usize])
                .map(|d| d.mag_sq());
            // for q in quads.clone() { println!("{:?}",q); };
            let mean = quads.clone().sum::<f32>() / count as f32;
            quads.map(|d| (d - mean) * (d - mean)).sum::<f32>() / (count as f32 - 1.0f32)
        }

        #[test]
        fn test_pyramid() {
            let p = Polyhedron::pyramid(Some(4));
            dumpobj(&p);
            assert!(p.faces().len() == 5);
            assert!(p.positions_len() == 5);
            assert!(p.to_edges().len() == 8);
        }

        #[test]
        fn test_cupola() {
            // let p = Polyhedron::square_cupola();
            let p = Polyhedron::pentagonal_cupola();
            dumpobj(&p);
            //        for n in [3, 4, 5] {
            //            let p = Polyhedron::cupola(Some(n));
            //            dumpobj(&p);
            let n = 5;
            let f = p.faces().len();
            let v = p.positions_len();
            let e = p.to_edges().len();
            assert!(f == n * 2 + 2);
            assert!(v == n + n * 2);
            assert!(e == n + n * 2 + n * 2);
            assert!(f + v - e == 2); // Euler's Formula
            assert!(edge_variance(&p) < 0.001);
            //        }
        }

        #[test]
        fn test_rotunda() {
            let p = Polyhedron::pentagonal_rotunda();
            dumpobj(&p);
            /*            let p = Polyhedron::rotunda();
                      dumpobj(&p);
                      let f = p.faces().len();
                      let v = p.positions_len();
                      let e = p.to_edges().len();
                      assert!(f == 17);
                      assert!(v == 5+5+10);
                      assert!(e == 5*5+5+5);
                      assert!(f + v - e == 2); // Euler's Formula
                      assert!(edge_variance(&p) < 0.001);
            */
        }

        #[test]
        fn test_variance() {
            assert!(edge_variance(&Polyhedron::hexahedron()) < 0.001);
            assert!(edge_variance(&Polyhedron::tetrahedron()) < 0.001);
            assert!(edge_variance(&Polyhedron::icosahedron()) < 0.001);
        }

        #[test]
        fn test_polyhedra() {
            for p in [
                Polyhedron::hexahedron(),
                Polyhedron::dodecahedron(),
                Polyhedron::tetrahedron(),
                Polyhedron::icosahedron(),
                Polyhedron::prism(Some(4)),
                Polyhedron::prism(Some(7)),
                Polyhedron::antiprism(Some(3)),
                Polyhedron::antiprism(Some(8)),
                Polyhedron::pyramid(Some(4)),
                Polyhedron::pyramid(Some(9)),
                Polyhedron::johnson(Some(3)),
                Polyhedron::johnson(Some(5)),
            ] {
                dumpobj(&p);
                let f = p.faces().len();
                let v = p.positions_len();
                let e = p.to_edges().len();
                assert!(f + v - e == 2); // Euler's Formula
            }
        }
    }
}
