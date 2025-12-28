use anchor_lang::prelude::*;

pub const PHI_INV: f64 = 0.618;

pub struct GenerativeKernel;
impl GenerativeKernel {
    pub fn icosahedral() -> Self { Self }
    pub fn generate_space(&self) -> TransformationSpace {
        TransformationSpace {
            pitch_classes: vec![PitchClass; 12],
            rhythmic_cells: vec![RhythmicCell; 20],
            timbral_morphs: vec![TimbralMorph; 30],
        }
    }
    pub fn random_transform(&self, point: &nalgebra::Vector3<f64>) -> nalgebra::Vector3<f64> { *point }
}

pub trait GenerativeKernelTrait {}
pub struct GroupElement;
pub struct Icosahedron;
#[derive(Clone, Copy)]
pub struct PitchClass;
#[derive(Clone, Copy)]
pub struct RhythmicCell;
#[derive(Clone, Copy)]
pub struct SymmetryGroup;
#[derive(Clone, Copy)]
pub struct TimbralMorph;

#[derive(Default)]
pub struct TransformationSpace {
    pub pitch_classes: Vec<PitchClass>,
    pub rhythmic_cells: Vec<RhythmicCell>,
    pub timbral_morphs: Vec<TimbralMorph>,
}
