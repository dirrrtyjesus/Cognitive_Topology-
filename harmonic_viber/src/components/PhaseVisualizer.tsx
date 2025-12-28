
import React, { useEffect, useRef } from 'react';
import { HarmonicSimulation, TAU } from '../logic/simulation';

interface Props {
    sim: HarmonicSimulation;
}

export const PhaseVisualizer: React.FC<Props> = ({ sim }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;
        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        const render = () => {
            // Resize
            const size = Math.min(window.innerWidth, 600);
            canvas.width = size;
            canvas.height = size;

            const cx = size / 2;
            const cy = size / 2;
            const radius = size * 0.35;

            // Clear (Transparent / Dark)
            ctx.clearRect(0, 0, size, size);

            // Draw Network Phase (Global)
            const globalPhase = sim.network.global_phase;
            const gx = cx + Math.cos(globalPhase) * (radius * 0.8);
            const gy = cy + Math.sin(globalPhase) * (radius * 0.8);

            // Global Phase Indicator
            ctx.beginPath();
            ctx.moveTo(cx, cy);
            ctx.lineTo(gx, gy);
            ctx.strokeStyle = 'rgba(147, 51, 234, 0.5)'; // Purple-600
            ctx.lineWidth = 4;
            ctx.stroke();

            // Draw Validators
            sim.validators.forEach((val) => {
                const x = cx + Math.cos(val.phase) * radius;
                const y = cy + Math.sin(val.phase) * radius;

                ctx.beginPath();
                ctx.arc(x, y, 4, 0, TAU);

                // Color based on resonance
                // High resonance = Gold, Low = Purple
                const isResonant = val.tau_k_resonance > 0.5;
                ctx.fillStyle = isResonant ? '#fbbf24' : '#a855f7'; // Amber-400 or Purple-500

                if (val.id === 'val-0') {
                    // Self (You)
                    ctx.fillStyle = '#22d3ee'; // Cyan
                    ctx.strokeStyle = '#fff';
                    ctx.lineWidth = 2;
                    ctx.stroke();
                    ctx.beginPath();
                    ctx.arc(x, y, 8, 0, TAU); // Larger for self
                    ctx.fillStyle = '#22d3ee';
                }

                ctx.fill();
            });

            // Draw Center Coherence Glow
            const coherence = sim.network.global_coherence; // 0 to 1
            const gradient = ctx.createRadialGradient(cx, cy, 0, cx, cy, radius * 0.5);
            gradient.addColorStop(0, `rgba(147, 51, 234, ${coherence * 0.5})`);
            gradient.addColorStop(1, 'rgba(0,0,0,0)');

            ctx.fillStyle = gradient;
            ctx.beginPath();
            ctx.arc(cx, cy, radius * 0.5, 0, TAU);
            ctx.fill();

            // requestAnimationFrame(render); 
        };

        render();
    }, [sim, sim.network.epoch]); // Re-render when epoch changes (simulation tick)

    return (
        <div className="flex justify-center items-center p-4">
            <canvas ref={canvasRef} className="rounded-full bg-black/20 backdrop-blur-sm border border-white/10" />
        </div>
    );
};
