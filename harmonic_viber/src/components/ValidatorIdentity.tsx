
import React from 'react';
import type { ValidatorVibe } from '../logic/simulation';
import { Fingerprint, Activity, Zap } from 'lucide-react';

interface Props {
    vibe: ValidatorVibe | undefined;
}

export const ValidatorIdentity: React.FC<Props> = ({ vibe }) => {
    if (!vibe) return <div className="glass-panel p-6 text-white/50">Loading Vibe...</div>;

    return (
        <div className="glass-panel p-6 rounded-xl border border-white/10 h-full">
            <div className="flex items-center gap-3 mb-6">
                <div className="p-3 bg-cyan-500/20 rounded-lg text-cyan-400">
                    <Fingerprint size={24} />
                </div>
                <div>
                    <h3 className="text-lg font-bold text-white font-mono">{vibe.id}</h3>
                    <div className="text-xs text-cyan-400/80 font-mono uppercase tracking-wider">Harmonic Validator</div>
                </div>
            </div>

            <div className="space-y-6">
                {/* Frequency */}
                <div>
                    <div className="flex justify-between text-sm mb-1">
                        <span className="text-white/60 flex items-center gap-2"><Activity size={14} /> Frequency (ω)</span>
                        <span className="text-white font-mono">{vibe.omega.toFixed(3)} Hz</span>
                    </div>
                    <div className="bg-white/10 h-1.5 rounded-full overflow-hidden">
                        <div className="h-full bg-cyan-400" style={{ width: `${(vibe.omega / 1.5) * 50}%` }} />
                    </div>
                </div>

                {/* Phase Stability */}
                <div>
                    <div className="flex justify-between text-sm mb-1">
                        <span className="text-white/60 flex items-center gap-2"><Activity size={14} /> Stability</span>
                        <span className="text-amber-400 font-mono">{(vibe.stability * 100).toFixed(1)}%</span>
                    </div>
                    <div className="bg-white/10 h-1.5 rounded-full overflow-hidden">
                        <div className="h-full bg-amber-400 transition-all duration-300"
                            style={{ width: `${vibe.stability * 100}%` }} />
                    </div>
                </div>

                {/* Overtones */}
                <div>
                    <div className="flex justify-between text-sm mb-2">
                        <span className="text-white/60 flex items-center gap-2"><Zap size={14} /> Overtone Structure</span>
                    </div>
                    <div className="flex gap-1 h-8 items-end">
                        {vibe.overtones.map((o, i) => (
                            <div
                                key={i}
                                className="flex-1 bg-purple-500/50 rounded-t-sm hover:bg-purple-400 transition-colors"
                                style={{ height: `${o * 100}%` }}
                            />
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
};
