
mod f64;
mod f32;
mod f16;

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
    fn add(&self, other: T) -> T;
    fn sub(&self, other: T) -> T;
    fn mul(&self, other: T) -> T;
    fn sqrt(&self) -> T;
    fn value(&self) -> T;
    fn zero() -> T;
    fn one() -> T;
    
}

trait Point<PseudoField> {
    fn coordinates(&self) -> Vec<PseudoField>;
}


fn distance<T: Copy + PseudoField<T>>(p1: &dyn Point<T>, p2: &dyn Point<T>) -> T {
    let coords1 = p1.coordinates();
    let coords2 = p2.coordinates();
    
    let mut sum = T::zero();
    for i in 0..coords1.len() {
        let diff = coords1[i].sub(coords2[i]);
        let squared = diff.mul(diff);
        sum = sum.add(squared);
    }

    sum.sqrt()
}


