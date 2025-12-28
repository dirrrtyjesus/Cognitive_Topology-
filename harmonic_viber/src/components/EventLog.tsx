import { LogEvent, LogEventType } from '../logic/simulation';

interface EventLogProps {
    events: LogEvent[];
}

const eventColors: Record<LogEventType, string> = {
    phase_lock: 'text-green-400',
    coherence_shift: 'text-cyan-400',
    nakamoto_update: 'text-purple-400',
    thicc_expansion: 'text-yellow-400',
    resonance_peak: 'text-amber-400',
    validator_golden: 'text-amber-300',
    entrainment: 'text-blue-400',
    residue_shed: 'text-red-400/70',
};

const eventIcons: Record<LogEventType, string> = {
    phase_lock: '◉',
    coherence_shift: '∿',
    nakamoto_update: '⎔',
    thicc_expansion: '◈',
    resonance_peak: '∞',
    validator_golden: '✧',
    entrainment: '⇌',
    residue_shed: '↯',
};

export function EventLog({ events }: EventLogProps) {
    return (
        <div className="glass-panel p-4 rounded-xl border border-white/5 text-xs font-mono h-[200px] overflow-hidden flex flex-col">
            <div className="text-purple-400 mb-2 flex items-center gap-2">
                <span className="animate-pulse">●</span>
                <span>{'>'} system.log_events()</span>
            </div>
            <div className="flex-1 overflow-y-auto space-y-1 scrollbar-thin scrollbar-thumb-white/10">
                {events.length === 0 ? (
                    <div className="text-white/30 italic">awaiting harmonic events...</div>
                ) : (
                    events.slice().reverse().map((event, i) => (
                        <div
                            key={`${event.epoch}-${i}`}
                            className={`${eventColors[event.type]} opacity-${Math.max(30, 100 - i * 10)}`}
                            style={{ opacity: Math.max(0.3, 1 - i * 0.08) }}
                        >
                            <span className="text-white/40">[{event.epoch.toFixed(1)}]</span>
                            {' '}
                            <span className="mr-1">{eventIcons[event.type]}</span>
                            {event.message}
                        </div>
                    ))
                )}
            </div>
        </div>
    );
}
