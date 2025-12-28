
import React from 'react';
import { PHI } from '../logic/simulation';
import type { NetworkState } from '../logic/simulation';
import { Activity, Radio, Scale, Zap } from 'lucide-react';

interface Props {
    network: NetworkState;
}

export const NetworkMetrics: React.FC<Props> = ({ network }) => {
    return (
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
            <div className="metric-card">
                <div className="flex items-center gap-2 text-purple-400 mb-2">
                    <Radio size={18} />
                    <span className="text-sm font-mono uppercase">Global Tau (τk)</span>
                </div>
                <div className="text-2xl font-bold font-mono text-white">
                    {network.network_tau_k.toFixed(3)}
                </div>
                <div className="text-xs text-white/50">Target: {(5.0 * PHI).toFixed(3)}</div>
            </div>

            <div className="metric-card">
                <div className="flex items-center gap-2 text-amber-400 mb-2">
                    <Activity size={18} />
                    <span className="text-sm font-mono uppercase">Coherence (R)</span>
                </div>
                <div className="text-2xl font-bold font-mono text-white">
                    {(network.global_coherence * 100).toFixed(1)}%
                </div>
                <div className="w-full bg-white/10 h-1 mt-2 rounded-full overflow-hidden">
                    <div
                        className="h-full bg-amber-400 transition-all duration-300"
                        style={{ width: `${network.global_coherence * 100}%` }}
                    />
                </div>
            </div>

            <div className="metric-card">
                <div className="flex items-center gap-2 text-cyan-400 mb-2">
                    <Scale size={18} />
                    <span className="text-sm font-mono uppercase">Nakamoto Coeff</span>
                </div>
                <div className="text-2xl font-bold font-mono text-white">
                    {network.nakamoto_coefficient.toFixed(1)}
                </div>
                <div className="text-xs text-white/50">Target: 50.0</div>
            </div>

            <div className="metric-card">
                <div className="flex items-center gap-2 text-pink-400 mb-2">
                    <Zap size={18} />
                    <span className="text-sm font-mono uppercase">Thicc NOW</span>
                </div>
                <div className="text-2xl font-bold font-mono text-white">
                    {network.thicc_now.toFixed(2)}
                </div>
                <div className="text-xs text-white/50">Temporal Depth</div>
            </div>
        </div>
    );
};
