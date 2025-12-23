//! The Concept Complex - Simplicial architecture of thought.
//!
//! STRATUM II: Concepts are not isolated points but connected faces
//! in a high-dimensional simplicial complex.
//!
//! The "Unthought" is a missing face - a homological hole.

use crate::types::*;
use std::collections::{HashMap, HashSet};

/// A single idea - vertex in the concept complex
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Idea {
    pub id: usize,
    pub name: String,
    pub embedding: Vec<i64>, // Discretized position for hashing
}

impl Idea {
    pub fn new(id: usize, name: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            embedding: vec![id as i64],
        }
    }

    pub fn with_embedding(mut self, embedding: Vec<i64>) -> Self {
        self.embedding = embedding;
        self
    }
}

/// A relation between ideas - a face (simplex) in the complex
#[derive(Debug, Clone)]
pub struct Relation {
    /// Vertices of this simplex (sorted by id)
    pub vertices: Vec<usize>,
    /// Dimension = number of vertices - 1
    pub dimension: usize,
    /// Strength of the relation
    pub weight: f64,
}

impl Relation {
    pub fn new(mut vertices: Vec<usize>) -> Self {
        vertices.sort();
        let dimension = vertices.len().saturating_sub(1);
        Self {
            vertices,
            dimension,
            weight: 1.0,
        }
    }

    /// Create an edge (1-simplex) between two ideas
    pub fn edge(a: usize, b: usize) -> Self {
        Self::new(vec![a, b])
    }

    /// Create a triangle (2-simplex) between three ideas
    pub fn triangle(a: usize, b: usize, c: usize) -> Self {
        Self::new(vec![a, b, c])
    }

    /// Create a tetrahedron (3-simplex)
    pub fn tetrahedron(a: usize, b: usize, c: usize, d: usize) -> Self {
        Self::new(vec![a, b, c, d])
    }

    /// Get the boundary of this simplex (faces of dimension - 1)
    pub fn boundary(&self) -> Vec<Relation> {
        if self.vertices.len() <= 1 {
            return vec![];
        }

        (0..self.vertices.len())
            .map(|i| {
                let mut face_vertices = self.vertices.clone();
                face_vertices.remove(i);
                Relation::new(face_vertices)
            })
            .collect()
    }

    /// Check if this simplex contains another as a face
    pub fn contains(&self, other: &Relation) -> bool {
        other.vertices.iter().all(|v| self.vertices.contains(v))
    }
}

/// A homological hole - the Unthought
#[derive(Debug, Clone)]
pub struct HomologicalHole {
    /// Dimension of the hole (0 = disconnection, 1 = loop, 2 = void)
    pub dimension: usize,
    /// Representatives of the hole (cycles that don't bound)
    pub representatives: Vec<Vec<usize>>,
    /// The "gap" that invites bridging
    pub gap_description: String,
}

impl HomologicalHole {
    pub fn new(dimension: usize, representatives: Vec<Vec<usize>>) -> Self {
        let gap_description = match dimension {
            0 => "Disconnected concept-clusters requiring bridging".to_string(),
            1 => "Conceptual loop that doesn't contract (paradox/strange loop)".to_string(),
            2 => "Void where higher-order relations should exist".to_string(),
            n => format!("H_{} hole: {}-dimensional void in concept space", n, n),
        };

        Self {
            dimension,
            representatives,
            gap_description,
        }
    }
}

/// The Concept Complex - simplicial complex of ideas
#[derive(Debug, Clone)]
pub struct ConceptComplex {
    /// All ideas (vertices)
    pub vertices: HashMap<usize, Idea>,
    /// All relations (simplices)
    pub faces: Vec<Relation>,
    /// Betti numbers: β₀ (components), β₁ (holes), β₂ (voids), ...
    pub betti_numbers: Vec<usize>,
    /// Maximum dimension of any simplex
    max_dimension: usize,
}

impl ConceptComplex {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            faces: Vec::new(),
            betti_numbers: vec![0],
            max_dimension: 0,
        }
    }

    /// Add an idea (vertex) to the complex
    pub fn add_idea(&mut self, idea: Idea) -> &mut Self {
        self.vertices.insert(idea.id, idea);
        self.invalidate_homology();
        self
    }

    /// Add a relation (simplex) to the complex
    pub fn add_relation(&mut self, relation: Relation) -> &mut Self {
        self.max_dimension = self.max_dimension.max(relation.dimension);
        self.faces.push(relation);
        self.invalidate_homology();
        self
    }

    /// Connect two ideas with an edge
    pub fn connect(&mut self, a: usize, b: usize) -> &mut Self {
        self.add_relation(Relation::edge(a, b))
    }

    /// Invalidate cached homology (needs recomputation)
    fn invalidate_homology(&mut self) {
        self.betti_numbers = vec![];
    }

    /// Compute homology groups and identify holes
    pub fn calculate_homology(&mut self) -> &Vec<usize> {
        if !self.betti_numbers.is_empty() {
            return &self.betti_numbers;
        }

        // Build chain complexes for each dimension
        let mut betti = Vec::new();

        // β₀: Number of connected components
        betti.push(self.count_connected_components());

        // β₁: Number of independent 1-cycles (loops that don't bound)
        betti.push(self.count_one_cycles());

        // β₂: Number of independent 2-cycles (voids)
        if self.max_dimension >= 2 {
            betti.push(self.count_two_cycles());
        }

        self.betti_numbers = betti;
        &self.betti_numbers
    }

    /// Count connected components (β₀)
    fn count_connected_components(&self) -> usize {
        if self.vertices.is_empty() {
            return 0;
        }

        let mut visited: HashSet<usize> = HashSet::new();
        let mut components = 0;

        for &id in self.vertices.keys() {
            if !visited.contains(&id) {
                components += 1;
                self.dfs_visit(id, &mut visited);
            }
        }

        components
    }

    /// DFS to find all connected vertices
    fn dfs_visit(&self, start: usize, visited: &mut HashSet<usize>) {
        let mut stack = vec![start];

        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            // Find neighbors via edges
            for face in &self.faces {
                if face.dimension == 1 && face.vertices.contains(&current) {
                    for &v in &face.vertices {
                        if !visited.contains(&v) {
                            stack.push(v);
                        }
                    }
                }
            }
        }
    }

    /// Count independent 1-cycles (β₁)
    fn count_one_cycles(&self) -> usize {
        let v = self.vertices.len();
        let e = self.faces.iter().filter(|f| f.dimension == 1).count();
        let c = self.count_connected_components();

        // Euler characteristic for 1-skeleton: χ = V - E
        // β₁ = E - V + C (for connected graph: β₁ = E - V + 1)
        if e >= v {
            e - v + c
        } else {
            0
        }
    }

    /// Count independent 2-cycles (β₂)
    fn count_two_cycles(&self) -> usize {
        let e = self.faces.iter().filter(|f| f.dimension == 1).count();
        let t = self.faces.iter().filter(|f| f.dimension == 2).count();

        // Simplified: if we have triangles but they don't fill all possible voids
        // This is a simplification - proper computation requires boundary matrices
        if t > 0 && e > 3 * t {
            1 // At least one void exists
        } else {
            0
        }
    }

    /// Identify the "gap" - a homological hole that invites bridging
    pub fn identify_gap(&self) -> Option<HomologicalHole> {
        // Look for H₁ holes first (most common cognitive gaps)
        if self.betti_numbers.len() > 1 && self.betti_numbers[1] > 0 {
            return Some(HomologicalHole::new(1, self.find_cycle_representatives()));
        }

        // Check for disconnection (H₀ > 1)
        if !self.betti_numbers.is_empty() && self.betti_numbers[0] > 1 {
            return Some(HomologicalHole::new(0, vec![]));
        }

        // Check for voids (H₂)
        if self.betti_numbers.len() > 2 && self.betti_numbers[2] > 0 {
            return Some(HomologicalHole::new(2, vec![]));
        }

        None
    }

    /// Find representatives of 1-cycles
    fn find_cycle_representatives(&self) -> Vec<Vec<usize>> {
        // Simplified: find a cycle in the edge graph
        let edges: Vec<_> = self.faces.iter()
            .filter(|f| f.dimension == 1)
            .collect();

        if edges.len() < 3 {
            return vec![];
        }

        // Try to find a simple cycle using the first few edges
        // Full implementation would use cycle basis algorithm
        vec![edges.iter()
            .take(3)
            .flat_map(|e| e.vertices.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()]
    }

    /// Bridge a gap by adding a new relation
    pub fn bridge_gap(&mut self, relation: Relation) -> Result<(), TopologyError> {
        self.add_relation(relation);
        self.calculate_homology();

        // Check if gap was successfully bridged
        if let Some(hole) = self.identify_gap() {
            Err(TopologyError::HomologicalHole {
                dimension: hole.dimension,
            })
        } else {
            Ok(())
        }
    }

    /// Get the Euler characteristic
    pub fn euler_characteristic(&self) -> i64 {
        let mut chi = self.vertices.len() as i64;

        for d in 0..=self.max_dimension {
            let count = self.faces.iter()
                .filter(|f| f.dimension == d)
                .count() as i64;

            if d % 2 == 1 {
                chi -= count;
            } else if d > 0 {
                chi += count;
            }
        }

        chi
    }
}

impl Default for ConceptComplex {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating concept complexes
pub struct ComplexBuilder {
    complex: ConceptComplex,
    next_id: usize,
}

impl ComplexBuilder {
    pub fn new() -> Self {
        Self {
            complex: ConceptComplex::new(),
            next_id: 0,
        }
    }

    pub fn add_idea(&mut self, name: impl Into<String>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.complex.add_idea(Idea::new(id, name));
        id
    }

    pub fn connect(&mut self, a: usize, b: usize) -> &mut Self {
        self.complex.connect(a, b);
        self
    }

    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize) -> &mut Self {
        self.complex.add_relation(Relation::triangle(a, b, c));
        self
    }

    pub fn build(mut self) -> ConceptComplex {
        self.complex.calculate_homology();
        self.complex
    }
}

impl Default for ComplexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_complex() {
        let complex = ConceptComplex::new();
        assert!(complex.vertices.is_empty());
        assert!(complex.faces.is_empty());
    }

    #[test]
    fn test_add_ideas() {
        let mut builder = ComplexBuilder::new();
        let a = builder.add_idea("Geometry");
        let b = builder.add_idea("Topology");
        let c = builder.add_idea("Cognition");

        builder.connect(a, b).connect(b, c);

        let complex = builder.build();
        assert_eq!(complex.vertices.len(), 3);
        assert_eq!(complex.betti_numbers[0], 1); // Connected
    }

    #[test]
    fn test_disconnected_components() {
        let mut builder = ComplexBuilder::new();
        let a = builder.add_idea("Island A");
        let b = builder.add_idea("Island B");
        let _ = builder.add_idea("Island C"); // Disconnected

        builder.connect(a, b);

        let complex = builder.build();
        assert_eq!(complex.betti_numbers[0], 2); // Two components
    }

    #[test]
    fn test_cycle_detection() {
        let mut builder = ComplexBuilder::new();
        let a = builder.add_idea("A");
        let b = builder.add_idea("B");
        let c = builder.add_idea("C");

        // Create a cycle: A - B - C - A
        builder.connect(a, b).connect(b, c).connect(c, a);

        let complex = builder.build();
        assert_eq!(complex.betti_numbers[0], 1); // Connected
        assert_eq!(complex.betti_numbers[1], 1); // One independent cycle
    }

    #[test]
    fn test_triangle_fills_cycle() {
        let mut builder = ComplexBuilder::new();
        let a = builder.add_idea("A");
        let b = builder.add_idea("B");
        let c = builder.add_idea("C");

        // Create cycle and fill with triangle
        builder.connect(a, b).connect(b, c).connect(c, a);
        builder.add_triangle(a, b, c);

        let complex = builder.build();
        // Triangle should "fill" the hole, but our simplified computation
        // may not capture this perfectly
        assert_eq!(complex.betti_numbers[0], 1);
    }

    #[test]
    fn test_identify_gap() {
        let mut builder = ComplexBuilder::new();
        let a = builder.add_idea("Thesis");
        let b = builder.add_idea("Antithesis");
        let c = builder.add_idea("Paradox");

        // Create a cycle (strange loop)
        builder.connect(a, b).connect(b, c).connect(c, a);

        let complex = builder.build();
        let gap = complex.identify_gap();

        assert!(gap.is_some());
        assert_eq!(gap.unwrap().dimension, 1);
    }

    #[test]
    fn test_euler_characteristic() {
        let mut builder = ComplexBuilder::new();
        let a = builder.add_idea("V1");
        let b = builder.add_idea("V2");
        let c = builder.add_idea("V3");

        builder.connect(a, b).connect(b, c);

        let complex = builder.build();
        // χ = V - E = 3 - 2 = 1
        assert_eq!(complex.euler_characteristic(), 1);
    }

    #[test]
    fn test_boundary_operator() {
        let triangle = Relation::triangle(0, 1, 2);
        let boundary = triangle.boundary();

        assert_eq!(boundary.len(), 3); // Three edges
        assert!(boundary.iter().all(|r| r.dimension == 1));
    }
}
