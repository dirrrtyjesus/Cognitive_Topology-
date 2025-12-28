use anchor_lang::prelude::*;

#[derive(Clone, Debug)]
pub struct Idea;
#[derive(Clone, Debug)]
pub struct Relation;

pub struct HomologicalHole{
    pub gap_description: String,
}

pub struct ConceptComplex {
    pub betti_numbers: Vec<usize>,
}

impl ConceptComplex {
    pub fn identify_gap(&self) -> Option<HomologicalHole> {
        Some(HomologicalHole { gap_description: "gap".to_string() })
    }
}

pub struct ComplexBuilder;

impl ComplexBuilder {
    pub fn new() -> Self { Self }
    pub fn add_idea(&mut self, _label: impl Into<String>) -> usize { 0 }
    pub fn connect(&mut self, _a: usize, _b: usize) -> &mut Self { self }
    pub fn build(self) -> ConceptComplex { ConceptComplex { betti_numbers: vec![1] } }
}
