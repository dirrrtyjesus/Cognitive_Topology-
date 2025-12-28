use crate::types::*;
use anchor_lang::prelude::*;

pub struct ConnectionForm;
pub struct HolonomyResult;

pub struct FiberBundle;
impl FiberBundle {
    pub fn curved(_dim: u8, _fiber_dim: u8, _curvature: f64) -> Self { Self }
}

#[derive(Clone, Copy)]
pub struct WorldPoint;
impl WorldPoint {
    pub fn new(_coords: Vec<f64>, _label: &str) -> Self { Self }
}

#[derive(Clone, Copy)]
pub struct Perspective;
impl Perspective {
    pub fn new(_dim: u8) -> Self { Self }
    pub fn with_state(self, _state: Vec<f64>) -> Self { self }
}

pub struct PerspectivalSelf;
impl PerspectivalSelf {
    pub fn new(_base_dim: u8, _fiber_dim: u8, _coupling: f64) -> Self { Self }
    pub fn emerge_at(&mut self, _base: WorldPoint, _fiber: Perspective) {}
    pub fn current(&self) -> Option<()> { Some(()) }
}
