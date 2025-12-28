
import React from 'react';
import type { RewardState } from '../logic/simulation';

interface Props {
    rewards: RewardState;
}

export const RewardsPanel: React.FC<Props> = ({ rewards }) => {
    const total = rewards.attunement + rewards.resonance + rewards.entrainment + rewards.thicc;

    return (
        <div className="glass-panel p-6 rounded-xl border border-white/10">
            <h3 className="text-xl font-mono text-white mb-6 flex items-center gap-2">
                <span className="text-purple-400">$</span> AUGMNTD Rewards
            </h3>

            <div className="space-y-4">
                <Row label="Attunement" value={rewards.attunement} color="text-purple-400" />
                <Row label="Resonance (τk)" value={rewards.resonance} color="text-amber-400" />
                <Row label="Entrainment" value={rewards.entrainment} color="text-cyan-400" />
                <Row label="ThiccNOW" value={rewards.thicc} color="text-pink-400" />

                <div className="h-px bg-white/10 my-4" />

                <div className="flex justify-between items-center">
                    <span className="font-mono text-white/70 uppercase">Total Epoch Reward</span>
                    <span className="font-mono text-2xl text-white font-bold">
                        {total.toLocaleString()}
                    </span>
                </div>
            </div>
        </div>
    );
};

const Row = ({ label, value, color }: { label: string, value: number, color: string }) => (
    <div className="flex justify-between items-center font-mono text-sm">
        <span className="text-white/60">{label}</span>
        <span className={color}>{value.toLocaleString()}</span>
    </div>
);
