// the Module include code, here, is the only place any concrete types should exist in the entire
// libs.rs code. If you used some concrete type in lib.rs , you know you done messed up and
// violated the foundational principles of the j92 project.
mod f64;
mod sym;
//mod iterator;
//mod f32;
//mod f16;
//mod rugfloat;
//mod rugrat;
// end of module include code

use std::rc::Rc;

pub enum JohnsonSolid {
    SquarePyramid,
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

trait PseudoField<T> {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn equal(&self, other: &Self) -> bool;
    fn sqrt(&self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
}

trait Point<T, U>
where
    U: PseudoField<T>,
{
    fn coordinates(&self) -> Box<dyn Iterator<Item = U>>;
}

trait Edge<X, T, U>
where
    X: Point<T, U>,
    U: PseudoField<T>,
{
    fn points(&self) -> Box<dyn Iterator<Item = X>>;
}

trait Face<Z, X, T, U>
where
    Z: Edge<X, T, U>,
    X: Point<T, U>,
    U: PseudoField<T> + Clone,
{
    fn edges(&self) -> Box<dyn Iterator<Item = Z>>;
    fn is_regular(&self)->bool {
    let ds: Vec<_>  = self.edges().map(|e| quadrance( &e.points().next().unwrap(),&e.points().next().unwrap())).collect();
    ds.iter().all( |d| d .equal ( &ds[0] )  ) 
    }
}

trait Polyhedron<M, Z, X, T, U>
where
    M: Face<Z, X, T, U>,
    Z: Edge<X, T, U>,
    X: Point<T, U>,
    U: PseudoField<T> + Clone,
{
    fn faces(&self) -> Box<dyn Iterator<Item = M>>;
}

fn quadrance<T, U>(p1: &dyn Point<T, U>, p2: &dyn Point<T, U>) -> U
where
    U: PseudoField<T> + Clone,
{
    p1.coordinates()
        .zip(p2.coordinates())
        .map(|(coord1, coord2)| coord1.sub(&coord2))
        .map(|x| x.mul(&x))
        .fold(U::zero(), |acc, val| acc.add(&val))
}

fn distance<T, U>(p1: &dyn Point<T, U>, p2: &dyn Point<T, U>) -> U
where
    U: PseudoField<T> + Clone,
{
    quadrance(p1, p2).sqrt()
}

/*fn is_regular_face<Z,X,T,U>(f:&dyn Face<Z, X, T, U>) -> bool
where
    Z: Edge<X, T, U>,
    X: Point<T, U>,
    U: PseudoField<T> + Clone,
{
}
*/
