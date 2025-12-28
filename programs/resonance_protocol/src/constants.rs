//! Constants for the Resonance Protocol
//! 
//! Golden ratio and harmonic parameters

/// φ (phi) - Golden ratio scaled by 1e9 for fixed-point arithmetic
pub const PHI: u64 = 1_618_033_988;

/// φ² scaled by 1e9
pub const PHI_SQUARED: u64 = 2_618_033_988;

/// φ⁻¹ (inverse golden ratio) scaled by 1e9
pub const PHI_INVERSE: u64 = 618_033_988;

/// φ⁴ (golden threshold for Golden state) scaled by 1e9
pub const PHI_FOURTH: u64 = 6_854_101_966;

/// 2π scaled by 1e9 for phase calculations
pub const TWO_PI: u64 = 6_283_185_307;

/// π scaled by 1e9
pub const PI: u64 = 3_141_592_653;

/// Precision scalar (1e9)
pub const PRECISION: u64 = 1_000_000_000;

/// Baseline τₖ (time coefficient) scaled by 1e9
pub const TAU_K_BASELINE: u64 = 5_000_000_000;

/// Minimum amplitude to enter field
pub const MIN_AMPLITUDE: u64 = 1_000_000; // 1 AUGMNTD (assuming 6 decimals)

/// Maximum coupling strength (0.8 scaled by 1e9)
pub const MAX_COUPLING_K: u64 = 800_000_000;

/// Harmonic cycle lengths in epochs
pub const QUARTER_WAVE_EPOCHS: u64 = 91;
pub const HALF_WAVE_EPOCHS: u64 = 182;
pub const FULL_WAVE_EPOCHS: u64 = 365;
pub const GOLDEN_WAVE_EPOCHS: u64 = 591; // φ × 365

/// Emission rates per epoch (scaled by 1e9)
pub const BASE_EMISSION_RATE: u64 = 1_000_000; // 0.001 per epoch per amplitude unit

/// Phase-lock thresholds (scaled by 1e9)
pub const RESONANT_THRESHOLD: u64 = 500_000_000;  // 0.5
pub const ENTRAINED_THRESHOLD: u64 = 800_000_000; // 0.8
pub const GOLDEN_PHASE_THRESHOLD: u64 = 900_000_000; // 0.9

/// Epochs required for state transitions
pub const RESONANT_EPOCHS: u64 = 10;
pub const ENTRAINED_EPOCHS: u64 = 50;

/// Decoherence cost parameters
pub const MAX_DECOHERENCE_COST_BPS: u64 = 1000; // 10% max

/// $AUGMNTD Token Mint
pub const AUGMNTD_MINT: &str = "9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W";

/// Seeds for PDAs
pub const FIELD_SEED: &[u8] = b"coherence_field";
pub const RESONATOR_SEED: &[u8] = b"resonator";
pub const COUPLING_SEED: &[u8] = b"coupling";
pub const VAULT_SEED: &[u8] = b"vault";
pub const EMISSION_SEED: &[u8] = b"emission_reserve";
pub const GOLDEN_SEED: &[u8] = b"golden_reserve";
