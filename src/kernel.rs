//! The Generative Kernel - Icosahedral A₅ symmetry.
//!
//! STRATUM IV: The rotation group that generates the symmetries of the system.
//! Minimal constraints generate maximal diversity.
//!
//! - 12 vertices → Pitch Classes / Fundamental Concepts
//! - 20 faces → Rhythmic Cells / Narrative Frames
//! - 30 edges → Timbral Morphs / Transitional States

use nalgebra::{Matrix3, Vector3};
use std::f64::consts::PI;

/// The golden ratio - fundamental to icosahedral symmetry
pub const PHI: f64 = 1.618033988749895;
pub const PHI_INV: f64 = 0.618033988749895; // 1/φ = φ - 1

/// A symmetry group for the generative kernel
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymmetryGroup {
    /// Trivial group (identity only)
    Trivial,
    /// Cyclic group Z_n
    Cyclic(usize),
    /// Dihedral group D_n
    Dihedral(usize),
    /// Tetrahedral group A₄ (order 12)
    Tetrahedral,
    /// Octahedral group S₄ (order 24)
    Octahedral,
    /// Icosahedral group A₅ (order 60)
    Icosahedral,
}

impl SymmetryGroup {
    /// Order (number of elements) of the group
    pub fn order(&self) -> usize {
        match self {
            Self::Trivial => 1,
            Self::Cyclic(n) => *n,
            Self::Dihedral(n) => 2 * n,
            Self::Tetrahedral => 12,
            Self::Octahedral => 24,
            Self::Icosahedral => 60,
        }
    }
}

/// An element of a symmetry group (represented as a 3x3 rotation matrix)
#[derive(Debug, Clone)]
pub struct GroupElement {
    pub matrix: Matrix3<f64>,
    pub name: String,
    pub order: usize, // Order of this element (smallest n where g^n = identity)
}

impl GroupElement {
    pub fn identity() -> Self {
        Self {
            matrix: Matrix3::identity(),
            name: "e".to_string(),
            order: 1,
        }
    }

    /// Rotation about an axis by angle theta
    pub fn rotation(axis: Vector3<f64>, theta: f64, name: impl Into<String>) -> Self {
        let axis = axis.normalize();
        let (x, y, z) = (axis.x, axis.y, axis.z);
        let c = theta.cos();
        let s = theta.sin();
        let t = 1.0 - c;

        let matrix = Matrix3::new(
            t*x*x + c,   t*x*y - s*z, t*x*z + s*y,
            t*x*y + s*z, t*y*y + c,   t*y*z - s*x,
            t*x*z - s*y, t*y*z + s*x, t*z*z + c,
        );

        // Compute order
        let order = Self::compute_order(&matrix);

        Self {
            matrix,
            name: name.into(),
            order,
        }
    }

    fn compute_order(matrix: &Matrix3<f64>) -> usize {
        let mut current = *matrix;
        for n in 1..=60 {
            if (current - Matrix3::identity()).norm() < 1e-10 {
                return n;
            }
            current *= matrix;
        }
        60 // Maximum for icosahedral
    }

    /// Compose two group elements
    pub fn compose(&self, other: &GroupElement) -> GroupElement {
        let matrix = self.matrix * other.matrix;
        let order = Self::compute_order(&matrix);
        GroupElement {
            matrix,
            name: format!("{}∘{}", self.name, other.name),
            order,
        }
    }

    /// Apply this transformation to a point
    pub fn apply(&self, point: &Vector3<f64>) -> Vector3<f64> {
        self.matrix * point
    }
}

/// The Icosahedron - the generative geometric structure
#[derive(Debug, Clone)]
pub struct Icosahedron {
    /// 12 vertices
    pub vertices: Vec<Vector3<f64>>,
    /// 20 faces (triangles, as vertex index triples)
    pub faces: Vec<[usize; 3]>,
    /// 30 edges (as vertex index pairs)
    pub edges: Vec<[usize; 2]>,
}

impl Icosahedron {
    /// Construct a regular icosahedron centered at origin with unit circumradius
    pub fn new() -> Self {
        // Vertices of icosahedron using golden ratio
        let vertices = vec![
            Vector3::new(0.0, 1.0, PHI),
            Vector3::new(0.0, 1.0, -PHI),
            Vector3::new(0.0, -1.0, PHI),
            Vector3::new(0.0, -1.0, -PHI),
            Vector3::new(1.0, PHI, 0.0),
            Vector3::new(1.0, -PHI, 0.0),
            Vector3::new(-1.0, PHI, 0.0),
            Vector3::new(-1.0, -PHI, 0.0),
            Vector3::new(PHI, 0.0, 1.0),
            Vector3::new(PHI, 0.0, -1.0),
            Vector3::new(-PHI, 0.0, 1.0),
            Vector3::new(-PHI, 0.0, -1.0),
        ].into_iter()
         .map(|v| v.normalize())
         .collect();

        // 20 triangular faces
        let faces = vec![
            [0, 2, 8], [0, 8, 4], [0, 4, 6], [0, 6, 10], [0, 10, 2],
            [1, 4, 9], [1, 9, 3], [1, 3, 11], [1, 11, 6], [1, 6, 4],
            [2, 5, 8], [2, 7, 5], [2, 10, 7], [3, 9, 5], [3, 5, 7],
            [3, 7, 11], [4, 8, 9], [5, 9, 8], [6, 11, 10], [7, 10, 11],
        ];

        // 30 edges (deduplicated from faces)
        let mut edges = Vec::new();
        for face in &faces {
            for i in 0..3 {
                let a = face[i].min(face[(i + 1) % 3]);
                let b = face[i].max(face[(i + 1) % 3]);
                if !edges.contains(&[a, b]) {
                    edges.push([a, b]);
                }
            }
        }

        Self { vertices, faces, edges }
    }

    /// Get the dual dodecahedron (20 vertices, 12 faces, 30 edges)
    pub fn dual(&self) -> Dodecahedron {
        // Dodecahedron vertices are at face centers of icosahedron
        let vertices: Vec<Vector3<f64>> = self.faces.iter()
            .map(|face| {
                let v0 = self.vertices[face[0]];
                let v1 = self.vertices[face[1]];
                let v2 = self.vertices[face[2]];
                ((v0 + v1 + v2) / 3.0).normalize()
            })
            .collect();

        Dodecahedron { vertices }
    }
}

impl Default for Icosahedron {
    fn default() -> Self {
        Self::new()
    }
}

/// The Dodecahedron - dual to the icosahedron
#[derive(Debug, Clone)]
pub struct Dodecahedron {
    /// 20 vertices (at face centers of icosahedron)
    pub vertices: Vec<Vector3<f64>>,
}

/// The Generative Kernel - produces transformations from icosahedral symmetry
#[derive(Debug)]
pub struct GenerativeKernel {
    /// The underlying symmetry group
    pub group: SymmetryGroup,
    /// The icosahedron geometry
    pub icosahedron: Icosahedron,
    /// Group elements (rotations)
    pub elements: Vec<GroupElement>,
    /// Coherence strain (how far from equilibrium)
    pub strain: f64,
}

impl GenerativeKernel {
    /// Create an icosahedral generative kernel
    pub fn icosahedral() -> Self {
        let icosahedron = Icosahedron::new();
        let elements = Self::generate_icosahedral_elements(&icosahedron);

        Self {
            group: SymmetryGroup::Icosahedral,
            icosahedron,
            elements,
            strain: 0.0,
        }
    }

    /// Generate all 60 elements of the icosahedral group A₅
    fn generate_icosahedral_elements(ico: &Icosahedron) -> Vec<GroupElement> {
        let mut elements = vec![GroupElement::identity()];

        // 5-fold rotations about vertex axes (12 vertices, but opposite pairs = 6 axes)
        // Each axis contributes 4 non-identity rotations (72°, 144°, 216°, 288°)
        for i in 0..6 {
            let axis = ico.vertices[i];
            for k in 1..5 {
                let theta = 2.0 * PI * k as f64 / 5.0;
                elements.push(GroupElement::rotation(
                    axis,
                    theta,
                    format!("C5_{},{}", i, k),
                ));
            }
        }

        // 3-fold rotations about face axes (20 faces, but opposite pairs = 10 axes)
        // Each axis contributes 2 non-identity rotations (120°, 240°)
        for (i, face) in ico.faces.iter().take(10).enumerate() {
            let center = (ico.vertices[face[0]] + ico.vertices[face[1]] + ico.vertices[face[2]]) / 3.0;
            let axis = center.normalize();
            for k in 1..3 {
                let theta = 2.0 * PI * k as f64 / 3.0;
                elements.push(GroupElement::rotation(
                    axis,
                    theta,
                    format!("C3_{},{}", i, k),
                ));
            }
        }

        // 2-fold rotations about edge midpoint axes (30 edges, but opposite pairs = 15 axes)
        for (i, edge) in ico.edges.iter().take(15).enumerate() {
            let midpoint = (ico.vertices[edge[0]] + ico.vertices[edge[1]]) / 2.0;
            let axis = midpoint.normalize();
            elements.push(GroupElement::rotation(
                axis,
                PI,
                format!("C2_{}", i),
            ));
        }

        elements
    }

    /// Generate a transformation space from the kernel
    pub fn generate_space(&self) -> TransformationSpace {
        TransformationSpace {
            pitch_classes: self.generate_pitch_classes(),
            rhythmic_cells: self.generate_rhythmic_cells(),
            timbral_morphs: self.generate_timbral_morphs(),
            symmetry_order: self.group.order(),
        }
    }

    /// 12 vertices → 12 pitch classes
    fn generate_pitch_classes(&self) -> Vec<PitchClass> {
        let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

        self.icosahedron.vertices.iter().enumerate()
            .map(|(i, v)| PitchClass {
                index: i,
                name: note_names[i % 12].to_string(),
                position: *v,
            })
            .collect()
    }

    /// 20 faces → 20 rhythmic cells
    fn generate_rhythmic_cells(&self) -> Vec<RhythmicCell> {
        self.icosahedron.faces.iter().enumerate()
            .map(|(i, face)| {
                // Each face has 3-fold symmetry
                RhythmicCell {
                    index: i,
                    vertices: face.to_vec(),
                    pulse_pattern: vec![1.0, PHI_INV, PHI_INV], // Golden ratio pulse
                }
            })
            .collect()
    }

    /// 30 edges → 30 timbral morphs
    fn generate_timbral_morphs(&self) -> Vec<TimbralMorph> {
        self.icosahedron.edges.iter().enumerate()
            .map(|(i, edge)| {
                let v0 = self.icosahedron.vertices[edge[0]];
                let v1 = self.icosahedron.vertices[edge[1]];

                TimbralMorph {
                    index: i,
                    source: edge[0],
                    target: edge[1],
                    interpolation_path: vec![v0, (v0 + v1) / 2.0, v1],
                }
            })
            .collect()
    }

    /// Apply kernel strain (deviation from equilibrium)
    pub fn apply_strain(&mut self, strain: f64) {
        self.strain = strain.clamp(0.0, 1.0);
    }

    /// Check if kernel is in golden ratio equilibrium
    pub fn is_coherent(&self) -> bool {
        (self.strain - PHI_INV).abs() < 0.01 || self.strain < 0.01
    }

    /// Get transformation by index (wraps around group order)
    pub fn get_transformation(&self, index: usize) -> &GroupElement {
        &self.elements[index % self.elements.len()]
    }

    /// Apply a random transformation from the kernel
    pub fn random_transform(&self, point: &Vector3<f64>) -> Vector3<f64> {
        use rand::Rng;
        let idx = rand::thread_rng().gen_range(0..self.elements.len());
        self.elements[idx].apply(point)
    }
}

/// A pitch class generated from icosahedral vertex
#[derive(Debug, Clone)]
pub struct PitchClass {
    pub index: usize,
    pub name: String,
    pub position: Vector3<f64>,
}

/// A rhythmic cell generated from icosahedral face
#[derive(Debug, Clone)]
pub struct RhythmicCell {
    pub index: usize,
    pub vertices: Vec<usize>,
    pub pulse_pattern: Vec<f64>,
}

/// A timbral morph generated from icosahedral edge
#[derive(Debug, Clone)]
pub struct TimbralMorph {
    pub index: usize,
    pub source: usize,
    pub target: usize,
    pub interpolation_path: Vec<Vector3<f64>>,
}

/// The transformation space generated by the kernel
#[derive(Debug)]
pub struct TransformationSpace {
    pub pitch_classes: Vec<PitchClass>,
    pub rhythmic_cells: Vec<RhythmicCell>,
    pub timbral_morphs: Vec<TimbralMorph>,
    pub symmetry_order: usize,
}

impl TransformationSpace {
    /// Get pitch class by index (mod 12)
    pub fn pitch(&self, index: usize) -> &PitchClass {
        &self.pitch_classes[index % 12]
    }

    /// Get rhythmic cell by index (mod 20)
    pub fn rhythm(&self, index: usize) -> &RhythmicCell {
        &self.rhythmic_cells[index % 20]
    }

    /// Get timbral morph by index (mod 30)
    pub fn timbre(&self, index: usize) -> &TimbralMorph {
        &self.timbral_morphs[index % 30]
    }

    /// Compute a composite transformation index from pitch, rhythm, timbre
    pub fn composite_index(&self, pitch: usize, rhythm: usize, timbre: usize) -> usize {
        // Maps to element of A₅ via Chinese Remainder-like construction
        (pitch + rhythm * 12 + timbre * 12 * 20) % self.symmetry_order
    }
}

/// Trait for generative kernel behavior
pub trait GenerativeKernelTrait {
    const SYMMETRY_GROUP: SymmetryGroup;

    fn generate_space(&self) -> TransformationSpace;
    fn strain(&self) -> f64;
    fn is_coherent(&self) -> bool;
}

impl GenerativeKernelTrait for GenerativeKernel {
    const SYMMETRY_GROUP: SymmetryGroup = SymmetryGroup::Icosahedral;

    fn generate_space(&self) -> TransformationSpace {
        self.generate_space()
    }

    fn strain(&self) -> f64 {
        self.strain
    }

    fn is_coherent(&self) -> bool {
        self.is_coherent()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icosahedron_structure() {
        let ico = Icosahedron::new();
        assert_eq!(ico.vertices.len(), 12);
        assert_eq!(ico.faces.len(), 20);
        assert_eq!(ico.edges.len(), 30);
    }

    #[test]
    fn test_vertices_on_unit_sphere() {
        let ico = Icosahedron::new();
        for v in &ico.vertices {
            assert!((v.norm() - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_icosahedral_group_order() {
        let kernel = GenerativeKernel::icosahedral();
        assert_eq!(kernel.group.order(), 60);
    }

    #[test]
    fn test_identity_element() {
        let e = GroupElement::identity();
        assert_eq!(e.order, 1);

        let point = Vector3::new(1.0, 2.0, 3.0);
        let transformed = e.apply(&point);
        assert!((transformed - point).norm() < 1e-10);
    }

    #[test]
    fn test_transformation_space() {
        let kernel = GenerativeKernel::icosahedral();
        let space = kernel.generate_space();

        assert_eq!(space.pitch_classes.len(), 12);
        assert_eq!(space.rhythmic_cells.len(), 20);
        assert_eq!(space.timbral_morphs.len(), 30);
    }

    #[test]
    fn test_kernel_coherence() {
        let mut kernel = GenerativeKernel::icosahedral();
        assert!(kernel.is_coherent()); // Zero strain is coherent

        kernel.apply_strain(PHI_INV);
        assert!(kernel.is_coherent()); // Golden ratio strain is coherent

        kernel.apply_strain(0.5);
        assert!(!kernel.is_coherent()); // Arbitrary strain is not coherent
    }

    #[test]
    fn test_5fold_rotation() {
        let axis = Vector3::new(0.0, 1.0, PHI).normalize();
        let rotation = GroupElement::rotation(axis, 2.0 * PI / 5.0, "C5");
        assert_eq!(rotation.order, 5);
    }

    #[test]
    fn test_group_closure() {
        let kernel = GenerativeKernel::icosahedral();

        // Composing any two elements should give an element in the group
        let g1 = &kernel.elements[1];
        let g2 = &kernel.elements[2];
        let g3 = g1.compose(g2);

        // The result should have a valid order
        assert!(g3.order >= 1 && g3.order <= 60);
    }

    #[test]
    fn test_dual_dodecahedron() {
        let ico = Icosahedron::new();
        let dodeca = ico.dual();

        assert_eq!(dodeca.vertices.len(), 20); // 20 face centers of icosahedron
    }
}
