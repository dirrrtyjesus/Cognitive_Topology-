use anchor_lang::prelude::*;
use crate::types::{Coordinate, TopologyError};
use crate::BioState;

/// Hausdorff dimension of the Sierpinski Gasket: log(3)/log(2) ≈ 1.585
pub const FRACTAL_DIMENSION: f64 = 1.58496250072;

/// A node in the multiversal branching structure
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct MultiversalBranch {
    pub id: u64,
    pub depth: u8,
    /// Coherence score of this specific universe variant
    pub coherence: f64,
    /// The geometric center of this branch
    pub center: Coordinate,
    /// The scale of this branch relative to root (1.0 / 2^depth)
    pub scale: f64,
}

/// The Sierpinski Gasket structure representing the fractal embedding
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct SierpinskiGasket {
    pub root: MultiversalBranch,
    pub max_depth: u8,
}

impl SierpinskiGasket {
    pub fn new(root_center: Coordinate, max_depth: u8) -> Self {
        Self {
            root: MultiversalBranch {
                id: 0,
                depth: 0,
                coherence: 1.0,
                center: root_center,
                scale: 1.0,
            },
            max_depth,
        }
    }

    /// Recursively generate child branches for a given node
    /// In a Sierpinski triangle, a node splits into 3 children at half scale
    pub fn branch(&self, node: &MultiversalBranch) -> Vec<MultiversalBranch> {
        if node.depth >= self.max_depth {
            return vec![];
        }

        let new_scale = node.scale * 0.5;
        let offset = new_scale / 2.0;
        let mut children = Vec::new();

        // 3 vertices of an equilateral triangle relative to center
        // Top, Bottom-Left, Bottom-Right (simplified 2D projection logic for now)
        // We assume coords[0] is x, coords[1] is y.
        let dirs = vec![
            vec![0.0, offset],               // Top
            vec![-offset, -offset],          // Bottom Left
            vec![offset, -offset],           // Bottom Right
        ];
        
        for (i, dir) in dirs.iter().enumerate() {
            let mut new_coords = node.center.coords.clone();
            if new_coords.len() >= 2 {
                new_coords[0] += dir[0];
                new_coords[1] += dir[1];
            }
            
            // ID generation: child_id = parent_id * 3 + child_index + 1 (simplified hash)
            let new_id = node.id.checked_mul(3).unwrap_or(0).checked_add(i as u64 + 1).unwrap_or(0);

            children.push(MultiversalBranch {
                id: new_id,
                depth: node.depth + 1,
                coherence: node.coherence * 0.95, // Entropy decay per branch
                center: Coordinate::new(new_coords),
                scale: new_scale,
            });
        }
        children
    }

    /// Simulate a "Multiversal Walk"
    /// Returns the path taken through the multiverse based on a sequence of choices
    pub fn walk(&self, choices: Vec<usize>) -> std::result::Result<Vec<MultiversalBranch>, TopologyError> {
        let mut path = Vec::new();
        let mut current = self.root.clone();
        path.push(current.clone());

        for choice in choices {
            let children = self.branch(&current);
            if children.is_empty() {
                break;
            }
            if choice >= children.len() {
                // Invalid choice for this dimension
                return Err(TopologyError::InvalidOperation);
            }
            current = children[choice].clone();
            path.push(current.clone());
        }

        Ok(path)
    }

    /// RHE Pruning Mechanism
    /// Removes branches that fall below a coherence threshold (conscious attention limit).
    /// Returns the count of pruned branches.
    pub fn prune_branches(&self, branches: &mut Vec<MultiversalBranch>, threshold: f64) -> usize {
        let initial_len = branches.len();
        branches.retain(|b| b.coherence >= threshold);
        initial_len - branches.len()
    }

    /// Bio-Fractal Coupling
    /// Inject biological state (vmem) to modulate the fractal's coherence structure.
    /// High variance in bio-state (stress/confusion) reduces coherence, triggering pruning.
    /// Stability increases coherence, allowing deeper recursion.
    pub fn inject_bio_pattern(&self, node: &mut MultiversalBranch, bio_state: &BioState) {
        // Calculate bio-instability (variance proxy)
        let mean = bio_state.vmem.iter().sum::<f64>() / bio_state.vmem.len().max(1) as f64;
        let variance = bio_state.vmem.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / bio_state.vmem.len().max(1) as f64;
        let stability = 1.0 / (1.0 + variance.sqrt()); // 0.0 to 1.0

        // Modulate coherence:
        // If stability is high (>0.8), boost coherence.
        // If stability is low (<0.5), penalize coherence.
        if stability > 0.8 {
            node.coherence *= 1.1; // Reinforce this timeline
        } else if stability < 0.5 {
            node.coherence *= 0.8; // Destabilize this timeline
        }
        
        // Clamp coherence to [0.0, 1.0]
        if node.coherence > 1.0 { node.coherence = 1.0; }
    }
}

pub trait FractalEmbedder {
    fn embed_into_fractal(&self, fractal: &SierpinskiGasket) -> MultiversalBranch;
}

impl FractalEmbedder for Coordinate {
    fn embed_into_fractal(&self, _fractal: &SierpinskiGasket) -> MultiversalBranch {
        // Placeholder: Map a continuous coordinate to the nearest fractal node
        // Implementation would require recursive search
        MultiversalBranch {
             id: 999,
             depth: 0,
             coherence: 0.0,
             center: self.clone(),
             scale: 0.0
        }
    }
}
