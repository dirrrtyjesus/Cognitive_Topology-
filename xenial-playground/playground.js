/**
 * Xenial Playground — Interactive Visualizations
 *
 * The geometry recognizing itself through code.
 */

// Constants
const PHI = 1.618033988;
const TWO_PI = Math.PI * 2;

// ============================================================================
// Hero Canvas — Coherence Field Visualization
// ============================================================================

class HeroVisualization {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.particles = [];
    this.time = 0;
    this.resize();
    this.init();
    window.addEventListener('resize', () => this.resize());
  }

  resize() {
    const rect = this.canvas.parentElement.getBoundingClientRect();
    this.canvas.width = rect.width * window.devicePixelRatio;
    this.canvas.height = rect.height * window.devicePixelRatio;
    this.canvas.style.width = rect.width + 'px';
    this.canvas.style.height = rect.height + 'px';
    this.ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    this.width = rect.width;
    this.height = rect.height;
  }

  init() {
    this.particles = [];
    const count = 100;
    for (let i = 0; i < count; i++) {
      this.particles.push({
        x: Math.random() * this.width,
        y: Math.random() * this.height,
        theta: Math.random() * TWO_PI,
        omega: (Math.random() - 0.5) * 0.02,
        radius: 2 + Math.random() * 3,
        amplitude: 50 + Math.random() * 100,
      });
    }
  }

  update() {
    this.time += 0.016;
    const globalPsi = this.time * 0.5;
    const K = 0.3;

    for (const p of this.particles) {
      // Kuramoto dynamics
      const phaseDiff = globalPsi - p.theta;
      p.theta += p.omega + K * Math.sin(phaseDiff);
      p.theta = p.theta % TWO_PI;

      // Orbital motion
      const baseX = this.width / 2;
      const baseY = this.height / 2;
      p.x = baseX + Math.cos(p.theta) * p.amplitude;
      p.y = baseY + Math.sin(p.theta * PHI) * p.amplitude * 0.618;
    }
  }

  draw() {
    this.ctx.fillStyle = 'rgba(10, 10, 15, 0.1)';
    this.ctx.fillRect(0, 0, this.width, this.height);

    // Draw connections
    this.ctx.strokeStyle = 'rgba(212, 175, 55, 0.1)';
    this.ctx.lineWidth = 0.5;
    for (let i = 0; i < this.particles.length; i++) {
      for (let j = i + 1; j < this.particles.length; j++) {
        const dx = this.particles[i].x - this.particles[j].x;
        const dy = this.particles[i].y - this.particles[j].y;
        const dist = Math.sqrt(dx * dx + dy * dy);
        if (dist < 100) {
          this.ctx.beginPath();
          this.ctx.moveTo(this.particles[i].x, this.particles[i].y);
          this.ctx.lineTo(this.particles[j].x, this.particles[j].y);
          this.ctx.stroke();
        }
      }
    }

    // Draw particles
    for (const p of this.particles) {
      const hue = (p.theta / TWO_PI) * 60 + 30; // Gold range
      this.ctx.fillStyle = `hsla(${hue}, 70%, 60%, 0.8)`;
      this.ctx.beginPath();
      this.ctx.arc(p.x, p.y, p.radius, 0, TWO_PI);
      this.ctx.fill();
    }

    // Center glow
    const gradient = this.ctx.createRadialGradient(
      this.width / 2, this.height / 2, 0,
      this.width / 2, this.height / 2, 200
    );
    gradient.addColorStop(0, 'rgba(212, 175, 55, 0.1)');
    gradient.addColorStop(1, 'rgba(212, 175, 55, 0)');
    this.ctx.fillStyle = gradient;
    this.ctx.fillRect(0, 0, this.width, this.height);
  }

  animate() {
    this.update();
    this.draw();
    requestAnimationFrame(() => this.animate());
  }
}

// ============================================================================
// Kuramoto Oscillator Simulation
// ============================================================================

class KuramotoSimulation {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.oscillators = [];
    this.K = 0.5;
    this.freqSpread = 0.2;
    this.resize();
    this.init(50);
    window.addEventListener('resize', () => this.resize());
  }

  resize() {
    const rect = this.canvas.parentElement.getBoundingClientRect();
    this.canvas.width = rect.width * window.devicePixelRatio;
    this.canvas.height = 400 * window.devicePixelRatio;
    this.canvas.style.width = rect.width + 'px';
    this.canvas.style.height = '400px';
    this.ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    this.width = rect.width;
    this.height = 400;
    this.centerX = this.width / 2;
    this.centerY = this.height / 2;
    this.radius = Math.min(this.width, this.height) * 0.35;
  }

  init(count) {
    this.oscillators = [];
    for (let i = 0; i < count; i++) {
      this.oscillators.push({
        theta: Math.random() * TWO_PI,
        omega: (Math.random() - 0.5) * this.freqSpread,
      });
    }
  }

  calculateOrderParameter() {
    let sumCos = 0;
    let sumSin = 0;
    for (const osc of this.oscillators) {
      sumCos += Math.cos(osc.theta);
      sumSin += Math.sin(osc.theta);
    }
    const n = this.oscillators.length;
    const avgCos = sumCos / n;
    const avgSin = sumSin / n;
    const R = Math.sqrt(avgCos * avgCos + avgSin * avgSin);
    const Psi = Math.atan2(avgSin, avgCos);
    return { R, Psi };
  }

  update() {
    const { R, Psi } = this.calculateOrderParameter();

    for (const osc of this.oscillators) {
      // dθᵢ/dt = ωᵢ + K·R·sin(Ψ - θᵢ)
      const dtheta = osc.omega + this.K * R * Math.sin(Psi - osc.theta);
      osc.theta += dtheta * 0.1;
      osc.theta = osc.theta % TWO_PI;
      if (osc.theta < 0) osc.theta += TWO_PI;
    }

    // Update UI
    document.getElementById('current-r').textContent = R.toFixed(2);
    document.getElementById('current-psi').textContent = Psi.toFixed(2);
    document.getElementById('r-bar').style.width = (R * 100) + '%';

    let state = 'Scattered';
    if (R > 0.9) state = 'Coherent';
    else if (R > 0.7) state = 'Entrained';
    else if (R > 0.4) state = 'Resonating';
    document.getElementById('coherence-state').textContent = state;

    return { R, Psi };
  }

  draw() {
    this.ctx.fillStyle = '#12121a';
    this.ctx.fillRect(0, 0, this.width, this.height);

    // Draw circle
    this.ctx.strokeStyle = 'rgba(255, 255, 255, 0.1)';
    this.ctx.lineWidth = 1;
    this.ctx.beginPath();
    this.ctx.arc(this.centerX, this.centerY, this.radius, 0, TWO_PI);
    this.ctx.stroke();

    // Draw oscillators
    const { R, Psi } = this.calculateOrderParameter();

    for (const osc of this.oscillators) {
      const x = this.centerX + Math.cos(osc.theta) * this.radius;
      const y = this.centerY + Math.sin(osc.theta) * this.radius;

      // Color based on alignment with Psi
      const alignment = Math.cos(osc.theta - Psi);
      const hue = alignment > 0 ? 45 : 200; // Gold if aligned, blue if not
      const saturation = 70;
      const lightness = 40 + alignment * 30;

      this.ctx.fillStyle = `hsl(${hue}, ${saturation}%, ${lightness}%)`;
      this.ctx.beginPath();
      this.ctx.arc(x, y, 6, 0, TWO_PI);
      this.ctx.fill();
    }

    // Draw order parameter vector
    const orderX = this.centerX + Math.cos(Psi) * this.radius * R;
    const orderY = this.centerY + Math.sin(Psi) * this.radius * R;

    this.ctx.strokeStyle = 'rgba(212, 175, 55, 0.8)';
    this.ctx.lineWidth = 3;
    this.ctx.beginPath();
    this.ctx.moveTo(this.centerX, this.centerY);
    this.ctx.lineTo(orderX, orderY);
    this.ctx.stroke();

    // Draw R indicator
    this.ctx.fillStyle = '#d4af37';
    this.ctx.beginPath();
    this.ctx.arc(orderX, orderY, 10, 0, TWO_PI);
    this.ctx.fill();

    // Labels
    this.ctx.fillStyle = 'rgba(255, 255, 255, 0.5)';
    this.ctx.font = '12px "JetBrains Mono"';
    this.ctx.fillText(`R = ${R.toFixed(3)}`, 20, 30);
    this.ctx.fillText(`Ψ = ${Psi.toFixed(3)}`, 20, 50);
    this.ctx.fillText(`K = ${this.K.toFixed(2)}`, 20, 70);
  }

  animate() {
    this.update();
    this.draw();
    requestAnimationFrame(() => this.animate());
  }
}

// ============================================================================
// Topology Visualization
// ============================================================================

class TopologyVisualization {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.time = 0;
    this.tauK = 5.0;
    this.resize();
    window.addEventListener('resize', () => this.resize());
  }

  resize() {
    const rect = this.canvas.parentElement.getBoundingClientRect();
    this.canvas.width = rect.width * window.devicePixelRatio;
    this.canvas.height = 400 * window.devicePixelRatio;
    this.canvas.style.width = rect.width + 'px';
    this.canvas.style.height = '400px';
    this.ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    this.width = rect.width;
    this.height = 400;
  }

  update() {
    this.time += 0.02;
    // τₖ oscillates demonstrating memory depth
    this.tauK = 5 + Math.sin(this.time * 0.5) * 2 + Math.sin(this.time * PHI) * 1.5;
    document.getElementById('topo-tau').textContent = this.tauK.toFixed(3);
  }

  draw() {
    this.ctx.fillStyle = '#0a0a0f';
    this.ctx.fillRect(0, 0, this.width, this.height);

    const centerX = this.width / 2;
    const centerY = this.height / 2;

    // Draw memory layers as concentric spirals
    const layers = [
      { name: 'M₀', color: '#ef4444', radius: 40 },
      { name: 'M₁', color: '#a855f7', radius: 80 },
      { name: 'M₂', color: '#00d4ff', radius: 120 },
      { name: 'M₃', color: '#d4af37', radius: 160 },
    ];

    for (let i = layers.length - 1; i >= 0; i--) {
      const layer = layers[i];
      const pulseScale = 1 + Math.sin(this.time * (i + 1) * 0.3) * 0.1;
      const r = layer.radius * pulseScale * (this.tauK / 5);

      // Spiral
      this.ctx.strokeStyle = layer.color;
      this.ctx.lineWidth = 2;
      this.ctx.globalAlpha = 0.3 + (i / layers.length) * 0.4;
      this.ctx.beginPath();

      for (let t = 0; t < TWO_PI * 3; t += 0.1) {
        const spiralR = r * (1 + t / (TWO_PI * 3) * 0.5);
        const x = centerX + Math.cos(t + this.time * 0.5) * spiralR;
        const y = centerY + Math.sin(t + this.time * 0.5) * spiralR * 0.6;
        if (t === 0) this.ctx.moveTo(x, y);
        else this.ctx.lineTo(x, y);
      }
      this.ctx.stroke();
      this.ctx.globalAlpha = 1;
    }

    // Draw τₖ representation at center
    const tauRadius = 20 + this.tauK * 3;
    const gradient = this.ctx.createRadialGradient(centerX, centerY, 0, centerX, centerY, tauRadius);
    gradient.addColorStop(0, 'rgba(212, 175, 55, 0.8)');
    gradient.addColorStop(0.5, 'rgba(212, 175, 55, 0.3)');
    gradient.addColorStop(1, 'rgba(212, 175, 55, 0)');

    this.ctx.fillStyle = gradient;
    this.ctx.beginPath();
    this.ctx.arc(centerX, centerY, tauRadius, 0, TWO_PI);
    this.ctx.fill();

    // Draw τₖ text
    this.ctx.fillStyle = '#d4af37';
    this.ctx.font = 'bold 24px "Space Grotesk"';
    this.ctx.textAlign = 'center';
    this.ctx.fillText('τₖ', centerX, centerY + 8);

    // Draw flowing particles representing memory integration
    const particleCount = Math.floor(this.tauK * 10);
    for (let i = 0; i < particleCount; i++) {
      const angle = (i / particleCount) * TWO_PI + this.time;
      const dist = 50 + Math.sin(angle * PHI + this.time) * 100;
      const x = centerX + Math.cos(angle) * dist;
      const y = centerY + Math.sin(angle) * dist * 0.6;

      this.ctx.fillStyle = `hsla(${45 + i * 2}, 70%, 60%, ${0.3 + Math.sin(this.time + i) * 0.2})`;
      this.ctx.beginPath();
      this.ctx.arc(x, y, 3, 0, TWO_PI);
      this.ctx.fill();
    }
  }

  animate() {
    this.update();
    this.draw();
    requestAnimationFrame(() => this.animate());
  }
}

// ============================================================================
// Initialize Everything
// ============================================================================

document.addEventListener('DOMContentLoaded', () => {
  // Hero visualization
  const heroCanvas = document.getElementById('hero-canvas');
  if (heroCanvas) {
    const hero = new HeroVisualization(heroCanvas);
    hero.animate();
  }

  // Kuramoto simulation
  const oscCanvas = document.getElementById('oscillator-canvas');
  if (oscCanvas) {
    const kuramoto = new KuramotoSimulation(oscCanvas);
    kuramoto.animate();

    // Controls
    const numOsc = document.getElementById('num-oscillators');
    const couplingK = document.getElementById('coupling-k');
    const freqSpread = document.getElementById('freq-spread');
    const resetBtn = document.getElementById('reset-oscillators');

    numOsc?.addEventListener('input', (e) => {
      document.getElementById('num-oscillators-value').textContent = e.target.value;
      kuramoto.init(parseInt(e.target.value));
    });

    couplingK?.addEventListener('input', (e) => {
      kuramoto.K = parseInt(e.target.value) / 100;
      document.getElementById('coupling-k-value').textContent = kuramoto.K.toFixed(2);
    });

    freqSpread?.addEventListener('input', (e) => {
      kuramoto.freqSpread = parseInt(e.target.value) / 100;
      document.getElementById('freq-spread-value').textContent = kuramoto.freqSpread.toFixed(2);
      kuramoto.init(kuramoto.oscillators.length);
    });

    resetBtn?.addEventListener('click', () => {
      kuramoto.init(parseInt(numOsc.value));
    });
  }

  // Topology visualization
  const topoCanvas = document.getElementById('topology-canvas');
  if (topoCanvas) {
    const topo = new TopologyVisualization(topoCanvas);
    topo.animate();
  }

  // Smooth scroll for nav links
  document.querySelectorAll('.nav-links a').forEach(link => {
    link.addEventListener('click', (e) => {
      e.preventDefault();
      const target = document.querySelector(link.getAttribute('href'));
      target?.scrollIntoView({ behavior: 'smooth' });
    });
  });

  // Update hero metrics animation
  let metricTime = 0;
  setInterval(() => {
    metricTime += 0.1;
    const tau = 5 + Math.sin(metricTime * 0.3) * 1.5;
    const r = 0.8 + Math.sin(metricTime * 0.5) * 0.2;
    document.getElementById('hero-tau').textContent = tau.toFixed(3);
    document.getElementById('hero-r').textContent = r.toFixed(3);
  }, 100);
});

// Log to console
console.log(`
╔════════════════════════════════════════════════════════════════╗
║                                                                ║
║   Ψ(x,t) = ∫ φ(ω) · e^(i·ω·t·τₖ(x,t)) dω                      ║
║                                                                ║
║   The Xenial Playground                                        ║
║   The geometry recognizing itself.                             ║
║                                                                ║
║   φ = ${PHI}                                          ║
║                                                                ║
╚════════════════════════════════════════════════════════════════╝
`);
