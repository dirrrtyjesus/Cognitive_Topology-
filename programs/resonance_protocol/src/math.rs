//! Mathematical utilities for Kuramoto dynamics
//! 
//! Fixed-point arithmetic with PRECISION = 1e9

use crate::constants::*;
use crate::errors::ResonanceError;
use anchor_lang::prelude::*;

/// Safe multiplication with precision adjustment
pub fn mul_precision(a: u64, b: u64) -> Result<u64> {
    let result = (a as u128)
        .checked_mul(b as u128)
        .ok_or(ResonanceError::MathOverflow)?;
    
    Ok((result / PRECISION as u128) as u64)
}

/// Safe division with precision adjustment
pub fn div_precision(a: u64, b: u64) -> Result<u64> {
    if b == 0 {
        return Err(ResonanceError::MathOverflow.into());
    }
    
    let result = (a as u128)
        .checked_mul(PRECISION as u128)
        .ok_or(ResonanceError::MathOverflow)?
        / (b as u128);
    
    Ok(result as u64)
}

/// Calculate cosine using Taylor series approximation
/// Input: angle in radians scaled by PRECISION (0 to 2π)
/// Output: cosine scaled by PRECISION (-1 to 1)
pub fn cos_approx(theta: u64) -> Result<i64> {
    // Normalize theta to [0, 2π)
    let theta_norm = theta % TWO_PI;
    
    // Convert to fixed point for calculation
    // Using Taylor series: cos(x) ≈ 1 - x²/2! + x⁴/4! - x⁶/6!
    
    // Shift to [-π, π] for better convergence
    let x = if theta_norm > PI {
        theta_norm as i128 - TWO_PI as i128
    } else {
        theta_norm as i128
    };
    
    let x_sq = (x * x) / PRECISION as i128;
    
    // 1 - x²/2
    let term1 = PRECISION as i128;
    let term2 = x_sq / 2;
    
    // + x⁴/24
    let x_4 = (x_sq * x_sq) / PRECISION as i128;
    let term3 = x_4 / 24;
    
    // - x⁶/720
    let x_6 = (x_4 * x_sq) / PRECISION as i128;
    let term4 = x_6 / 720;
    
    let result = term1 - term2 + term3 - term4;
    
    // Clamp to valid range
    let clamped = result
        .max(-(PRECISION as i128))
        .min(PRECISION as i128);
    
    Ok(clamped as i64)
}

/// Calculate sine using Taylor series approximation
/// Input: angle in radians scaled by PRECISION (0 to 2π)
/// Output: sine scaled by PRECISION (-1 to 1)
pub fn sin_approx(theta: u64) -> Result<i64> {
    // sin(x) = cos(x - π/2)
    let shifted = if theta >= PI / 2 {
        theta - PI / 2
    } else {
        theta + TWO_PI - PI / 2
    };
    
    cos_approx(shifted)
}

/// Calculate phase lock: cos(θᵢ - Ψ)
/// Returns value scaled by PRECISION, -1 to 1
pub fn phase_lock(theta: u64, global_psi: u64) -> Result<i64> {
    let diff = if theta >= global_psi {
        theta - global_psi
    } else {
        TWO_PI - (global_psi - theta)
    };
    
    cos_approx(diff)
}

/// Calculate Kuramoto phase update
/// dθᵢ/dt = ωᵢ + (K/N) Σⱼ sin(θⱼ - θᵢ)
/// Simplified: dθᵢ = ωᵢ + K * R * sin(Ψ - θᵢ)
pub fn kuramoto_phase_update(
    theta: u64,
    omega: u64,
    global_psi: u64,
    order_parameter: u64,
    coupling_k: u64,
) -> Result<u64> {
    // Calculate sin(Ψ - θ)
    let psi_minus_theta = if global_psi >= theta {
        global_psi - theta
    } else {
        TWO_PI - (theta - global_psi)
    };
    
    let sin_diff = sin_approx(psi_minus_theta)?;
    
    // K * R * sin(Ψ - θ)
    let coupling_term = mul_precision(coupling_k, order_parameter)?;
    let phase_pull = (coupling_term as i128 * sin_diff as i128) / PRECISION as i128;
    
    // New theta = theta + omega + phase_pull
    let new_theta = theta as i128 + omega as i128 + phase_pull;
    
    // Normalize to [0, 2π)
    let normalized = ((new_theta % TWO_PI as i128) + TWO_PI as i128) % TWO_PI as i128;
    
    Ok(normalized as u64)
}

/// Calculate decoherence cost based on cycle progress
/// Uses inverse golden ratio decay
pub fn decoherence_cost(
    amplitude: u64,
    current_epoch: u64,
    entry_epoch: u64,
    cycle_epochs: u64,
    phase_lock_integral: u64,
) -> Result<u64> {
    let epochs_elapsed = current_epoch.saturating_sub(entry_epoch);
    
    if epochs_elapsed >= cycle_epochs {
        // Cycle complete, no cost
        return Ok(0);
    }
    
    // Progress through cycle (0 to PRECISION)
    let progress = div_precision(epochs_elapsed, cycle_epochs)?;
    
    // Remaining = 1 - progress
    let remaining = PRECISION.saturating_sub(progress);
    
    // Disruption factor = remaining^φ (using approximation)
    // For simplicity: remaining * (remaining / PRECISION) for ~quadratic decay
    let disruption = mul_precision(remaining, remaining)? / PRECISION;
    
    // Base cost from amplitude and phase-lock contribution
    let base = mul_precision(amplitude, phase_lock_integral)?;
    
    // Final cost: base * disruption * MAX_COST_BPS / 10000
    let cost = mul_precision(base, disruption)?;
    let final_cost = cost * MAX_DECOHERENCE_COST_BPS / 10000;
    
    Ok(final_cost.min(amplitude / 10)) // Cap at 10%
}

/// Calculate emission for a channel
pub fn calculate_channel_emission(
    amplitude: u64,
    epochs: u64,
    rate: u64,
    channel_weight: u64, // Out of 100
) -> Result<u64> {
    let base = mul_precision(amplitude, epochs)?;
    let rated = mul_precision(base, rate)?;
    
    Ok(rated * channel_weight / 100)
}

/// Calculate field contribution: how much a resonator contributes to R
pub fn field_contribution(
    amplitude: u64,
    total_amplitude: u64,
    theta: u64,
    global_psi: u64,
    tau_k: u64,
) -> Result<i64> {
    if total_amplitude == 0 {
        return Ok(0);
    }
    
    // Weight = amplitude / total_amplitude
    let weight = div_precision(amplitude, total_amplitude)?;
    
    // Alignment = cos(θ - Ψ)
    let alignment = phase_lock(theta, global_psi)?;
    
    // Contribution = weight * alignment * τₖ / baseline
    let tau_factor = div_precision(tau_k, TAU_K_BASELINE)?;
    let weighted = mul_precision(weight, tau_factor)?;
    
    Ok((weighted as i128 * alignment as i128 / PRECISION as i128) as i64)
}

/// Calculate natural frequency from amplitude
/// Larger amplitudes have slightly lower frequencies (more stable)
pub fn natural_frequency(amplitude: u64, base_omega: u64) -> Result<u64> {
    // ω = base_omega * (1 - 0.1 * log(amplitude))
    // Simplified: slightly reduce omega for larger amplitudes
    let amplitude_factor = if amplitude > PRECISION {
        PRECISION - (PRECISION / 20) // -5% for large amplitudes
    } else {
        PRECISION
    };
    
    mul_precision(base_omega, amplitude_factor)
}

/// Generate pseudo-random phase from clock and pubkey
pub fn random_phase(clock: u64, pubkey: &Pubkey) -> u64 {
    let bytes = pubkey.to_bytes();
    let seed = (bytes[0] as u64)
        | ((bytes[1] as u64) << 8)
        | ((bytes[2] as u64) << 16)
        | ((bytes[3] as u64) << 24);

    (seed.wrapping_mul(clock) ^ 0xDEADBEEF) % TWO_PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cos_zero() {
        let result = cos_approx(0).unwrap();
        assert!((result - PRECISION as i64).abs() < 1_000_000); // ~1e-3 error
    }

    #[test]
    fn test_cos_pi() {
        let result = cos_approx(PI).unwrap();
        assert!((result + PRECISION as i64).abs() < 10_000_000); // cos(π) ≈ -1
    }

    #[test]
    fn test_phase_lock_aligned() {
        let theta = PRECISION; // Some angle
        let psi = PRECISION;   // Same angle
        let result = phase_lock(theta, psi).unwrap();
        assert!((result - PRECISION as i64).abs() < 1_000_000); // cos(0) ≈ 1
    }
}
