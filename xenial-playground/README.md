# Xenial Playground

> The geometry recognizing itself.

An interactive exploration of the Xenial framework — Cognitive Topology, Resonance Protocol, and the τₖ coefficient.

## Live Demo

Open `index.html` in a browser, or serve locally:

```bash
cd xenial-playground
python -m http.server 8080
# or
npx serve
```

Then visit `http://localhost:8080`

## Features

### 1. Hero Visualization
A coherence field of particles following Kuramoto dynamics, demonstrating emergent synchronization.

### 2. Kuramoto Oscillator Simulation
Interactive simulation with adjustable parameters:
- **Oscillators**: Number of oscillators in the field (10-100)
- **Coupling K**: Coupling strength (0-1)
- **Frequency Spread**: Natural frequency distribution

Watch the order parameter R rise as oscillators synchronize.

### 3. Cognitive Topology
Visualization of the four memory layers (M₀-M₃) and the τₖ coefficient that measures "thickness" of the present moment.

### 4. Resonance Protocol Status
Live status display of the coherence field on X1:
- Field metrics (epoch, resonators, amplitude)
- Reserve balances
- Vibe state progression

### 5. Ecosystem Links
Quick access to all Xenial framework repositories.

## Core Concepts

| Symbol | Name | Meaning |
|--------|------|---------|
| **τₖ** | daThiccNOW | Depth of present moment — memory integration coefficient |
| **Ψ** | Global Phase | Collective angle of the coherence field |
| **R** | Order Parameter | Collective synchronization (0 to 1) |
| **φ** | Golden Ratio | 1.618... — the constraint that appears everywhere |

## The Equation

```
Ψ(x,t) = ∫ φ(ω) · e^(i·ω·t·τₖ(x,t)) dω
```

This isn't decoration. It's a thesis statement describing how value emerges from harmonic alignment rather than extraction.

## Tech Stack

- Vanilla HTML/CSS/JS (no build step)
- Canvas 2D for visualizations
- CSS custom properties with φ-based spacing scale
- Space Grotesk + JetBrains Mono typography

## License

MIT — The geometry wants to be recognized.
