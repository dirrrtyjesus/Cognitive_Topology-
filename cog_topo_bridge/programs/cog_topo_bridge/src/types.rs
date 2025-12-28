use anchor_lang::prelude::*;

pub const PHI: f64 = 1.618033988749895;

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct AttentionTensor {
    pub raw: Vec<f64>,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct CognitiveLoad {
    pub val: f64,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Coordinate {
    pub coords: Vec<f64>,
}

impl Coordinate {
    pub fn new(coords: Vec<f64>) -> Self {
        Self { coords }
    }
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Geodesic {}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Path {}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Thought {
    pub coherence: f64,
    pub content: String,
}

#[error_code]
pub enum TopologyError {
    #[msg("Singularity detected in cognitive manifold.")]
    Singularity,
    #[msg("Invalid topology operation.")]
    InvalidOperation,
}
