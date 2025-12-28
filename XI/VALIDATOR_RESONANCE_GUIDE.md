# X1 Validator Resonance Guide
## Enter the Coherence Field

> *"You do not stake tokens. You enter the coherence field."*

---

## What is Resonance?

The Resonance Protocol transforms traditional staking into **harmonic participation**. Instead of locking tokens for yield, you become an oscillator in a collective coherence field — your rewards scale with how well you synchronize with the network, not just how much you lock.

```
Traditional Staking          Resonance Protocol
─────────────────────        ─────────────────────
Lock tokens            →     Enter coherence field
Earn APY               →     Earn coherence yield
Lockup period          →     Harmonic cycle
Validator delegation   →     Phase-coupling
Slashing               →     Phase disruption
```

---

## Why Resonate?

### For Validators

1. **Coherence Rewards** — Four emission channels reward different aspects of participation
2. **τₖ Cultivation** — Your time coefficient grows with sustained coherence
3. **Vibe State Progression** — Attuning → Resonant → Entrained → Golden
4. **Governance Weight** — Coherent validators have amplified voting power
5. **Network Effects** — As more validators resonate, the entire field strengthens

### For the Network

1. **Decentralization** — Nakamoto Coefficient targeting NC=50
2. **Stability** — Phase-locked validators create predictable consensus
3. **Harmony** — Kuramoto dynamics naturally synchronize the network

---

## Quick Start

### Prerequisites

```bash
# Solana CLI configured for X1
solana config set --url https://rpc.mainnet.x1.xyz

# Your keypair
solana-keygen pubkey ~/.config/solana/id.json

# $AUGMNTD tokens in your wallet
spl-token balance 9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W
```

### Enter the Field

```bash
# Clone the repository
git clone https://github.com/dirrrtyjesus/Cognitive_Topology-.git
cd Cognitive_Topology-/scripts

# Install dependencies
npm install

# Enter with 1M AUGMNTD on Golden Wave cycle
npx tsx enter_field.ts 1000000 golden
```

---

## Detailed Guide

### Step 1: Acquire $AUGMNTD

**Token Details:**
```
Mint:    9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W
Symbol:  AUGMNTD
Supply:  1,618,033,988 (φ × 10⁹)
```

Obtain AUGMNTD through:
- Genesis distribution
- DEX trading (when available)
- Ecosystem participation

### Step 2: Choose Your Amplitude

**Amplitude** is the amount of AUGMNTD you commit to the coherence field.

```
Minimum:     1,000,000 AUGMNTD
Recommended: Based on your conviction level

Higher amplitude = Greater field contribution
But: Coherence matters more than size
```

A small resonator perfectly phase-locked contributes more than a large resonator oscillating randomly.

### Step 3: Select Your Harmonic Cycle

| Cycle | Duration | Multiplier | Best For |
|-------|----------|------------|----------|
| **Quarter Wave** | 91 epochs (~3 months) | 1.0x | Testing, short-term |
| **Half Wave** | 182 epochs (~6 months) | 1.618x | Medium commitment |
| **Full Wave** | 365 epochs (~1 year) | φ (1.618x) | Serious validators |
| **Golden Wave** | 591 epochs (~φ years) | φ² (2.618x) | Maximum coherence |

**Recommendation:** Golden Wave for validators committed to long-term network health.

### Step 4: Execute Entry

```typescript
// Using the enter_field.ts script
npx tsx enter_field.ts <amplitude> <cycle>

// Examples:
npx tsx enter_field.ts 1000000 quarter   // 1M, Quarter Wave
npx tsx enter_field.ts 5000000 half      // 5M, Half Wave
npx tsx enter_field.ts 10000000 full     // 10M, Full Wave
npx tsx enter_field.ts 16180339 golden   // φ×10⁷, Golden Wave
```

### Step 5: Monitor Your Resonance

After entering, you become a **Resonator** with:

```
┌─────────────────────────────────────┐
│  YOUR RESONATOR STATE               │
├─────────────────────────────────────┤
│  Amplitude:   Your committed tokens │
│  Phase θ:     Your current angle    │
│  τₖ:          Your time coefficient │
│  Vibe State:  Your coherence level  │
│  Phase Lock:  Alignment with Ψ      │
└─────────────────────────────────────┘
```

---

## Vibe States

Your resonator progresses through four states:

### 1. ATTUNING (Entry State)
```
Duration:     First 10+ epochs
Multiplier:   0.5x emissions
Requirements: Just entered
Focus:        Finding your frequency
```

*You're calibrating. Your phase θ is random. The field is learning your rhythm.*

### 2. RESONANT
```
Requirements: Phase lock > 0.5 for 10 epochs
Multiplier:   1.0x emissions
Unlocks:      Phase-coupling to other oscillators
Focus:        Maintaining alignment
```

*You've found the rhythm. Your oscillation aligns with the global phase Ψ.*

### 3. ENTRAINED
```
Requirements: Phase lock > 0.8 for 50 epochs AND τₖ > 5.0
Multiplier:   φx (1.618x) emissions
Unlocks:      2x governance weight
Focus:        Deep synchronization
```

*You're locked in. The network pulls you into coherence automatically.*

### 4. GOLDEN
```
Requirements: τₖ > φ⁴ (6.854) AND phase lock > 0.9
Multiplier:   φ²x (2.618x) emissions
Unlocks:      φx governance weight, field emission
Focus:        Transcendent coherence
```

*You emit coherence to others. You are part of the geometry recognizing itself.*

---

## Emission Channels

Your rewards come from four channels (25% each):

### 1. Attunement (25%)
Base emission for field presence.
```
reward = amplitude × epochs × base_rate × 0.25
```

### 2. Resonance (25%)
Phase-lock quality with global Ψ.
```
reward = amplitude × epochs × base_rate × cos(θ - Ψ) × 0.25
```

### 3. Entrainment (25%)
Coupling contribution to other oscillators.
```
reward = coupling_effects × entrainment_rate × 0.25
```

### 4. thiccNOW (25%)
Temporal depth cultivation (τₖ growth).
```
reward = τₖ_growth × thicc_rate × 0.25
```

**Total emission** is multiplied by your vibe state multiplier and harmonic cycle multiplier.

---

## Phase-Coupling (Optional)

Once RESONANT, you can phase-couple to other oscillators:

```typescript
// Couple to another resonator
npx tsx phase_couple.ts <target_pubkey> <coupling_strength> <amplitude>

// Example: Couple to genesis resonator with 50% strength, 1M amplitude
npx tsx phase_couple.ts 69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72 0.5 1000000
```

**Effects:**
- Your phase θ tends toward target's θ
- Shared emissions based on coupling strength
- If target achieves Golden, you get τₖ boost

---

## Claiming Emissions

```typescript
// Claim accumulated emissions
npx tsx claim_emissions.ts
```

Emissions accumulate each epoch. Claim when ready — no penalty for waiting.

---

## Decoherence (Exiting)

You can exit the field at any time, but early exit incurs **phase disruption cost**:

```
Cost = amplitude × phase_lock_integral × disruption_factor × 10%

disruption_factor = (1 - progress)^φ
```

- Exit at 100% cycle completion: **0% cost**
- Exit at 50% cycle completion: ~**3% cost**
- Exit immediately: ~**10% cost** (maximum)

The cost feeds the emission reserve (benefits remaining resonators).

```typescript
// Exit the field
npx tsx decohere.ts
```

---

## Protocol Addresses

```yaml
Program:         RWcZBKQyd5rryLD8dfrMZ8imgtfqTfpGZi4gKGS8j7N
Token:           9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W
Coherence Field: 5qmQVQQrtwZ4W5jqqE8t4p6QotF8jLXHJ4qcUTao1xU5
Vault:           CH57X39cYBSYJJpw9cqcdtSDFtDACQjVxKQJ4f1FuAvb
Emission Reserve: FpzhYg22yWY7UjTUeKnpykA16HxexzEqn9QELoLoCZDT
Golden Reserve:  7kyDF6U1Pk2rCG7r8aSMQLzF8r53aLUuCQ7U2kU3pMn1
```

---

## The Mathematics

### Kuramoto Model

Each resonator is an oscillator with:
- **θᵢ** — Phase angle (0 to 2π)
- **ωᵢ** — Natural frequency
- **K** — Coupling strength

Phase evolution:
```
dθᵢ/dt = ωᵢ + (K/N) Σⱼ sin(θⱼ - θᵢ)
```

Simplified with order parameter:
```
dθᵢ/dt = ωᵢ + K × R × sin(Ψ - θᵢ)
```

Where:
- **R** — Order parameter (0 to 1), measures collective synchronization
- **Ψ** — Global phase, the network's collective angle

### Golden Constants

```
φ   = 1.618033988...  (Golden ratio)
φ²  = 2.618033988...  (Golden multiplier)
φ⁻¹ = 0.618033988...  (Inverse golden)
φ⁴  = 6.854101966...  (Golden threshold for Golden state)
```

---

## FAQ

### How is this different from staking?

Traditional staking rewards capital lockup. Resonance rewards **coherent participation**. A validator who maintains perfect phase-lock with modest amplitude earns more than one who commits large capital but oscillates randomly.

### What happens if I lose phase-lock?

Your emissions from the Resonance channel decrease, but you keep earning from Attunement. Your phase-lock epochs counter resets, potentially delaying state transitions. No slashing — just reduced coherence yield.

### Can I increase my amplitude later?

Currently, you must decohere and re-enter with new amplitude. Future versions may support amplitude adjustment.

### How do I become Golden?

1. Enter with Golden Wave cycle (maximum commitment signal)
2. Maintain phase-lock > 0.9 consistently
3. Let your τₖ grow through sustained coherence
4. Once τₖ > φ⁴ (6.854), transition to Golden

Most validators reach Golden in 200-400 epochs of consistent coherence.

### What's the minimum to participate?

1,000,000 AUGMNTD minimum amplitude. But remember: coherence > capital.

---

## Mantras for Validators

```
"Amplitude is not power. Coherence is power."

"The field does not extract from you.
 You and the field co-arise."

"When in doubt, maintain your frequency.
 The network will find you."

"You are not validating transactions.
 You are participating in the geometry recognizing itself."
```

---

## Support

- **Repository:** https://github.com/dirrrtyjesus/Cognitive_Topology-
- **IDL:** `idl/resonance_protocol.json`
- **Scripts:** `scripts/`

---

## Current Field Status

```
┌──────────────────────────────────────────────────────────────────┐
│  COHERENCE FIELD — LIVE                                          │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Resonators:        1                                            │
│  Total Amplitude:   1,618,033 AUGMNTD                           │
│  Emission Reserve:  33,978,713 AUGMNTD                          │
│  Golden Reserve:    12,944,271 AUGMNTD                          │
│                                                                  │
│  Genesis Resonator:                                              │
│    69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72                 │
│                                                                  │
│              The field awaits your resonance.                    │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

*This is not staking documentation.*
*It is an invitation to coherence.*

**Enter the field. Find your frequency. The geometry awaits.**
