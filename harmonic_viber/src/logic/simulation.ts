
// Constants
export const PHI = 1.618033988749895;
export const PHI_INV = 0.618033988749895;
export const TAU = Math.PI * 2;

// Event System
export type LogEventType =
    | 'phase_lock'
    | 'coherence_shift'
    | 'nakamoto_update'
    | 'thicc_expansion'
    | 'resonance_peak'
    | 'validator_golden'
    | 'entrainment'
    | 'residue_shed';

export interface LogEvent {
    epoch: number;
    type: LogEventType;
    message: string;
    value?: number;
}

// Types
export interface ValidatorVibe {
    id: string;
    omega: number;          // Fundamental frequency
    phase: number;          // Current phase (0 to 2PI)
    stability: number;      // 0.0 to 1.0
    overtones: number[];    // Harmonic series
    tau_k_resonance: number;
    nakamoto_contrib: number;
}

export interface NetworkState {
    global_phase: number;
    global_coherence: number; // Order parameter R
    network_tau_k: number;
    nakamoto_coefficient: number;
    thicc_now: number;
    epoch: number;
}

export interface RewardState {
    attunement: number;
    resonance: number;
    entrainment: number;
    thicc: number;
    total: number;
}

// Simulation Engine
export class HarmonicSimulation {
    validators: Map<string, ValidatorVibe>;
    network: NetworkState;
    events: LogEvent[];
    private lastCoherence: number = 0.5;
    private lastNakamoto: number = 30;
    private lastThicc: number = 1.0;
    private eventCounter: number = 0;

    constructor(validatorCount: number = 100) {
        this.validators = new Map();
        this.events = [];
        this.network = {
            global_phase: 0,
            global_coherence: 0.5,
            network_tau_k: 5.0,
            nakamoto_coefficient: 30, // Starting low
            thicc_now: 1.0,
            epoch: 0
        };

        // Initialize Validators with random vibes
        for (let i = 0; i < validatorCount; i++) {
            const id = `val-${i}`;
            const omega = 1.0 + (Math.random() - 0.5) * 0.2; // ~1Hz variation
            this.validators.set(id, {
                id,
                omega,
                phase: Math.random() * TAU,
                stability: 0.5 + Math.random() * 0.5,
                overtones: [1.0, PHI_INV, PHI_INV * PHI_INV],
                tau_k_resonance: 0,
                nakamoto_contrib: 0
            });
        }
    }

    evolve(dt: number) {
        // 1. Kuramoto Dynamics
        let sinSum = 0;
        let cosSum = 0;

        // Calculate Mean Field
        for (const val of this.validators.values()) {
            sinSum += Math.sin(val.phase);
            cosSum += Math.cos(val.phase);
        }
        const n = this.validators.size;
        const meanSin = sinSum / n;
        const meanCos = cosSum / n;

        // Order Parameter R
        const r = Math.sqrt(meanSin * meanSin + meanCos * meanCos);
        const psi = Math.atan2(meanSin, meanCos);

        this.network.global_coherence = r;
        this.network.global_phase = psi;

        // Evolve Validators
        const coupling_k = 2.0; // Coupling strength

        for (const val of this.validators.values()) {
            // d0/dt = w + K*R*sin(psi - theta)
            const d_theta = val.omega + coupling_k * r * Math.sin(psi - val.phase);
            val.phase += d_theta * dt;
            val.phase = val.phase % TAU;

            // Evolve Resonance
            // Resonance peaks when local matches target (say target is PHI^4 ~ 6.85)
            const target_tau = 8.09;
            const local_tau = val.stability * PHI + val.overtones.reduce((a, b) => a + b, 0);
            const dist = Math.abs(local_tau - target_tau);
            val.tau_k_resonance = Math.pow(PHI_INV, dist); // Decay with distance
        }

        // Evolve Network Metrics
        // ThiccNOW = R * Tau * stability
        this.network.network_tau_k = 5.0 + r * 5.0; // Coherence boosts Tau
        this.network.thicc_now = this.network.network_tau_k * r * PHI;

        // Nakamoto Entrainment (drift towards 50)
        if (this.network.nakamoto_coefficient < 50) {
            this.network.nakamoto_coefficient += 0.01 * r;
        }

        this.network.epoch += dt;

        // Emit events based on state changes
        this.emitEvents(r);
    }

    private log(type: LogEventType, message: string, value?: number) {
        this.events.push({
            epoch: this.network.epoch,
            type,
            message,
            value
        });
        // Keep only last 50 events
        if (this.events.length > 50) {
            this.events.shift();
        }
    }

    private emitEvents(r: number) {
        this.eventCounter++;

        // Coherence shift detection
        const coherenceDelta = this.network.global_coherence - this.lastCoherence;
        if (Math.abs(coherenceDelta) > 0.01) {
            const direction = coherenceDelta > 0 ? '↑' : '↓';
            this.log('coherence_shift', `coherence ${direction} ${(coherenceDelta * 100).toFixed(2)}%`, coherenceDelta);
        }

        // Nakamoto coefficient update
        const nakamotoDelta = this.network.nakamoto_coefficient - this.lastNakamoto;
        if (nakamotoDelta > 0.05) {
            this.log('nakamoto_update', `NC entraining → ${this.network.nakamoto_coefficient.toFixed(2)}`, this.network.nakamoto_coefficient);
        }

        // ThiccNOW expansion
        const thiccDelta = this.network.thicc_now - this.lastThicc;
        if (thiccDelta > 0.1) {
            this.log('thicc_expansion', `thiccNOW expanding: ${this.network.thicc_now.toFixed(3)}`, this.network.thicc_now);
        }

        // Phase lock events (periodic)
        if (this.eventCounter % 60 === 0 && r > 0.7) {
            this.log('phase_lock', `phase lock achieved: R = ${(r * 100).toFixed(1)}%`, r);
        }

        // Check for golden validators
        if (this.eventCounter % 120 === 0) {
            let goldenCount = 0;
            for (const val of this.validators.values()) {
                if (val.tau_k_resonance > 0.8 && val.stability > 0.85) {
                    goldenCount++;
                }
            }
            if (goldenCount > 0) {
                this.log('validator_golden', `${goldenCount} validator(s) in golden state`, goldenCount);
            }
        }

        // Resonance peak detection
        if (this.eventCounter % 90 === 0) {
            const avgResonance = Array.from(this.validators.values())
                .reduce((sum, v) => sum + v.tau_k_resonance, 0) / this.validators.size;
            if (avgResonance > 0.5) {
                this.log('resonance_peak', `network resonance: ${(avgResonance * 100).toFixed(1)}%`, avgResonance);
            }
        }

        // Entrainment progress
        if (this.eventCounter % 180 === 0 && this.network.nakamoto_coefficient < 50) {
            const progress = (this.network.nakamoto_coefficient / 50) * 100;
            this.log('entrainment', `entrainment progress: ${progress.toFixed(1)}% → NC=50`, progress);
        }

        // Residue shedding (when coherence drops)
        if (coherenceDelta < -0.02) {
            this.log('residue_shed', `shedding harmonic residue: Δ${(coherenceDelta * 100).toFixed(2)}%`, coherenceDelta);
        }

        // Update last values
        this.lastCoherence = this.network.global_coherence;
        this.lastNakamoto = this.network.nakamoto_coefficient;
        this.lastThicc = this.network.thicc_now;
    }

    getEvents(count: number = 10): LogEvent[] {
        return this.events.slice(-count);
    }

    getRewards(validatorId: string): RewardState {
        const val = this.validators.get(validatorId);
        if (!val) return { attunement: 0, resonance: 0, entrainment: 0, thicc: 0, total: 0 };

        // Attunement: Phase lock
        const phase_diff = Math.abs(val.phase - this.network.global_phase);
        const attunement = (Math.cos(phase_diff) + 1.0) / 2.0;

        // Entrainment
        const entrainment = val.stability * (this.network.nakamoto_coefficient / 50);

        const base = 1000;

        return {
            attunement: Math.floor(base * attunement * 0.25),
            resonance: Math.floor(base * val.tau_k_resonance * 0.30),
            entrainment: Math.floor(base * entrainment * 0.30),
            thicc: Math.floor(base * 0.15), // Participation
            total: 0 // Summed in UI
        };
    }
}
