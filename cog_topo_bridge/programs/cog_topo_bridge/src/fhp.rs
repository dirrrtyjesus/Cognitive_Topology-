use anchor_lang::prelude::*;
use std::f64::consts::PI;

pub const FUNDAMENTAL_FREQ: f64 = 432.0;
pub const SEED_65_STRAIN: f64 = 1.0;
pub const DT: f64 = 0.01; // Time step for simulation

/// The "Time-Key": Temporal curvature derived from system coherence.
/// High Coherence = High TauK (Golden Time).
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize, Copy)]
pub struct TauK {
    pub value: f64,
}

impl TauK {
    pub fn new(coherence: f64) -> Self {
        const PHI: f64 = 1.618033988749895;
        // Logic: Time accelerates/dilates with coherence.
        // Golden Time is achieved when R (coherence) is high.
        Self { value: coherence * PHI }
    }
}

/// A probabilistic bit representing "It from Bit".
/// State is maintained as probability amplitudes (alpha, beta).
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct TauQubit {
    pub alpha: (f64, f64), // Real, Imaginary parts of alpha
    pub beta: (f64, f64),  // Real, Imaginary parts of beta
}

impl TauQubit {
    pub fn new_superposition() -> Self {
        let inv_sqrt_2 = 1.0 / 2.0_f64.sqrt();
        Self {
            alpha: (inv_sqrt_2, 0.0),
            beta: (inv_sqrt_2, 0.0),
        }
    }
}

/// Network of coupled oscillators following Kuramoto dynamics.
/// dθ_i/dt = ω_i + (K/N) * Σ sin(θ_j - θ_i)
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct KuramotoNetwork {
    pub phases: Vec<f64>,
    pub frequencies: Vec<f64>,
    pub coupling_k: f64,
}

impl KuramotoNetwork {
    pub fn new(size: usize, coupling_k: f64) -> Self {
        // Initialize with random phases (simulated deterministically here for now)
        // In productin, use a PRNG or oracle input
        let phases = (0..size).map(|i| (i as f64 * 0.5) % (2.0 * PI)).collect();
        let frequencies = vec![FUNDAMENTAL_FREQ; size]; // All resonate at 432Hz initially
        
        Self {
            phases,
            frequencies,
            coupling_k,
        }
    }

    /// Advance the simulation by one time step (Euler method)
    pub fn step(&mut self) {
        let n = self.phases.len() as f64;
        let old_phases = self.phases.clone();

        for i in 0..self.phases.len() {
            let mut interaction_sum = 0.0;
            for j in 0..self.phases.len() {
                interaction_sum += (old_phases[j] - old_phases[i]).sin();
            }
            
            let d_theta = self.frequencies[i] + (self.coupling_k / n) * interaction_sum;
            self.phases[i] = (self.phases[i] + d_theta * DT) % (2.0 * PI);
        }
    }

    /// Calculate the Order Parameter (Coherence R)
    /// R = |(1/N) * Σ e^(iθ_j)|
    pub fn order_parameter(&self) -> f64 {
        let n = self.phases.len() as f64;
        let sum_cos: f64 = self.phases.iter().map(|theta| theta.cos()).sum();
        let sum_sin: f64 = self.phases.iter().map(|theta| theta.sin()).sum();
        
        let r_squared = (sum_cos.powi(2) + sum_sin.powi(2)) / n.powi(2);
        r_squared.sqrt()
    }
}

/// A field containing multiple scales of oscillator networks (Fractal resonance).
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct MultiScaleField {
    pub layers: Vec<KuramotoNetwork>,
}

impl MultiScaleField {
    pub fn new(scales: usize) -> Self {
        let mut layers = Vec::new();
        for i in 0..scales {
            // Each scale can have different characteristics
             let coupling = 1.0 + (i as f64 * 0.5); // Higher scales couple tighter?
             layers.push(KuramotoNetwork::new(10, coupling));
        }
        Self { layers }
    }
    
    pub fn coherence_across_scales(&self) -> f64 {
        let total_r: f64 = self.layers.iter().map(|l| l.order_parameter()).sum();
        total_r / self.layers.len() as f64
    }
}

// Placeholder structs from previous version kept for compatibility if needed
pub struct Seed65;
pub struct FragmentType;
pub struct PathwayDistribution;
pub struct CoherenceResult;
pub struct BridgeResult;
pub struct TemporalScale;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kuramoto_sync() {
        // Weak coupling, should maintain some disorder or slow sync
        let mut net = KuramotoNetwork::new(10, 0.1); 
        // Manually scramble phases to ensure we aren't starting at perfect sync
        net.phases = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 0.5, 1.5, 2.5]; 
        
        let initial_r = net.order_parameter();
        
        // Increase coupling to force sync
        net.coupling_k = 5.0; 
        
        for _ in 0..100 {
            net.step();
        }
        
        let final_r = net.order_parameter();
        assert!(final_r > initial_r, "System should synchronize over time with high coupling");
        assert!(final_r > 0.8, "System should reach high coherence");
    }

    #[test]
    fn test_tau_k_derivation() {
        let coherence = 1.0;
        let tau = TauK::new(coherence);
        // 1.0 * 1.618...
        assert!((tau.value - 1.6180339887).abs() < 1e-5);
        
        let zero_coherence = 0.0;
        let tau_z = TauK::new(zero_coherence);
        assert_eq!(tau_z.value, 0.0);
    }
    
    #[test]
    fn test_multiscale_field() {
        let field = MultiScaleField::new(3);
        assert_eq!(field.layers.len(), 3);
        
        let avg_r = field.coherence_across_scales();
        assert!(avg_r >= 0.0 && avg_r <= 1.0);
    }
}
