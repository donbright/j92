// the Module include code, here, is the only place any concrete types should exist in the entire
// libs.rs code. If you used some concrete type in lib.rs , you know you done messed up and
// violated the foundational principles of the j92 project.
mod f64;
//mod f32;
//mod f16;
//mod rugfloat;
//mod rugrat;
// end of module include code

use std::rc::Rc;


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

trait PseudoField<T> {
    fn add(&self, other: &Self) -> Self;
    fn sub(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
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

trait Edge<T, U, X>
where
    T: Point<U, X>,
    X: PseudoField<U>,
{
    fn points(&self) -> Box<dyn Iterator<Item = T>>;
}

/*
trait Face<T,R,S> where T: Edge<T,R> , R: Point<S> {
    fn edges(&self) -> Box<dyn Iterator<Item = T>>;
}


trait Polyhedron<T,R> where T: Face<T,R> + Edge<T,R> + Point<R> {
    fn faces(&self) -> Box<dyn Iterator<Item = T>>;
}
*/



fn distance<T, U>(p1: &dyn Point<T, U>, p2: &dyn Point<T, U>) -> U
where
    U: PseudoField<T> + Clone,
{
    p1.coordinates()
        .zip(p2.coordinates())
        .map(|(coord1, coord2)| coord1.sub(&coord2))
        .map(|x|x.mul(&x))
        .fold(U::zero(), |acc, val| acc.add(&val))
        .sqrt()
}


