//! Error types for the Resonance Protocol

use anchor_lang::prelude::*;

#[error_code]
pub enum ResonanceError {
    /// Amplitude below minimum threshold
    #[msg("Amplitude below minimum threshold")]
    AmplitudeTooLow,

    /// Insufficient amplitude for operation
    #[msg("Insufficient amplitude")]
    InsufficientAmplitude,

    /// Coupling strength exceeds maximum
    #[msg("Coupling strength exceeds maximum (0.8)")]
    CouplingTooStrong,

    /// Cannot couple while in Attuning state
    #[msg("Cannot phase-couple while still attuning")]
    StillAttuning,

    /// Already coupled to this target
    #[msg("Already phase-coupled to this target")]
    AlreadyCoupled,

    /// Not coupled to this target
    #[msg("Not phase-coupled to this target")]
    NotCoupled,

    /// Cannot decohere before minimum period
    #[msg("Cannot decohere before minimum period")]
    TooEarlyToDecohere,

    /// Invalid phase value
    #[msg("Phase value out of valid range")]
    InvalidPhase,

    /// Invalid vibe state transition
    #[msg("Invalid vibe state transition")]
    InvalidVibeTransition,

    /// Not eligible for state transition
    #[msg("Not eligible for vibe state transition")]
    NotEligibleForTransition,

    /// Unauthorized oracle update
    #[msg("Only oracle can update field state")]
    UnauthorizedOracle,

    /// Unauthorized authority action
    #[msg("Only authority can perform this action")]
    UnauthorizedAuthority,

    /// No emissions to claim
    #[msg("No emissions available to claim")]
    NoEmissionsToClaim,

    /// Math overflow
    #[msg("Mathematical overflow")]
    MathOverflow,

    /// Math underflow
    #[msg("Mathematical underflow")]
    MathUnderflow,

    /// Invalid harmonic cycle
    #[msg("Invalid harmonic cycle")]
    InvalidHarmonicCycle,

    /// Field not initialized
    #[msg("Coherence field not initialized")]
    FieldNotInitialized,

    /// Resonator not found
    #[msg("Resonator not found")]
    ResonatorNotFound,

    /// Cannot self-couple
    #[msg("Cannot phase-couple to yourself")]
    CannotSelfCouple,

    /// Target not a valid resonator
    #[msg("Coupling target is not a valid resonator")]
    InvalidCouplingTarget,

    /// Epoch mismatch
    #[msg("Epoch mismatch - field needs update")]
    EpochMismatch,

    /// Already in Golden state
    #[msg("Already in Golden state")]
    AlreadyGolden,

    /// Reserve depleted
    #[msg("Emission reserve depleted")]
    ReserveDepleted,
}
