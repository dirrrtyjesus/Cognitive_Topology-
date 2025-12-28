
import { useEffect, useRef, useState } from 'react'
import { HarmonicSimulation } from './logic/simulation'
import type { NetworkState, RewardState, ValidatorVibe, LogEvent } from './logic/simulation'
import { NetworkMetrics } from './components/NetworkMetrics'
import { PhaseVisualizer } from './components/PhaseVisualizer'
import { ValidatorIdentity } from './components/ValidatorIdentity'
import { RewardsPanel } from './components/RewardsPanel'
import { EventLog } from './components/EventLog'
import { Wallet, ShieldCheck, Loader2 } from 'lucide-react'

function App() {
  // Use Ref for simulation engine to persist across renders without re-init
  const simRef = useRef<HarmonicSimulation>(new HarmonicSimulation(12)); // 12 Validators (Music)

  // State for React Reactivity
  const [network, setNetwork] = useState<NetworkState>(simRef.current.network);
  const [myVibe, setMyVibe] = useState<ValidatorVibe | undefined>(undefined);
  const [rewards, setRewards] = useState<RewardState>({ attunement: 0, resonance: 0, entrainment: 0, thicc: 0, total: 0 });
  const [events, setEvents] = useState<LogEvent[]>([]);

  // X1 Connection State
  const [connected, setConnected] = useState(false);
  const [connecting, setConnecting] = useState(false);

  const connectWallet = () => {
    setConnecting(true);
    // Simulate X1 handshake
    setTimeout(() => {
      setConnected(true);
      setConnecting(false);
    }, 1500);
  };

  useEffect(() => {
    let frameId: number;

    const loop = () => {
      // Only evolve if connected (or maybe background ticks are fine, but let's pause for effect)
      if (connected) {
        simRef.current.evolve(0.016); // ~60fps dt
      }

      setNetwork({ ...simRef.current.network });
      setMyVibe({ ...simRef.current.validators.get('val-0')! });
      setRewards(simRef.current.getRewards('val-0'));
      setEvents([...simRef.current.getEvents(15)]);

      frameId = requestAnimationFrame(loop);
    };

    frameId = requestAnimationFrame(loop);
    return () => cancelAnimationFrame(frameId);
  }, [connected]);

  if (!connected) {
    return (
      <div className="min-h-screen bg-[#05030a] text-white flex flex-col items-center justify-center p-4 relative overflow-hidden">
        {/* Background ambient glow */}
        <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-purple-900/20 blur-[100px] rounded-full pointer-events-none" />

        <div className="glass-panel p-8 md:p-12 rounded-2xl border border-white/10 max-w-md w-full text-center relative z-10 flex flex-col items-center gap-6">
          <div className="w-16 h-16 bg-gradient-to-br from-cyan-400 to-purple-600 rounded-xl flex items-center justify-center shadow-lg shadow-purple-500/30 mb-2">
            <ShieldCheck size={32} className="text-white" />
          </div>

          <div>
            <h1 className="text-2xl font-bold font-mono text-white mb-2">Harmonic Viber</h1>
            <p className="text-white/50 text-sm">X1 Network Mainnet</p>
          </div>

          <p className="text-white/60 text-sm leading-relaxed">
            Connect your X1 Wallet to synchronize with the global harmonic field.
            Zero-cost voting enabled. Residue free.
          </p>

          <button
            onClick={connectWallet}
            disabled={connecting}
            className="w-full py-3 px-4 bg-white/10 hover:bg-white/20 border border-white/10 rounded-lg font-mono text-cyan-400 transition-all flex items-center justify-center gap-2 group disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {connecting ? (
              <>
                <Loader2 size={18} className="animate-spin" />
                Connecting to X1...
              </>
            ) : (
              <>
                <Wallet size={18} className="group-hover:scale-110 transition-transform" />
                Connect Wallet
              </>
            )}
          </button>

          <div className="mt-4 text-[10px] text-white/20 font-mono uppercase tracking-widest">
            Nakamoto Entrainment Protocol v1.0
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-[#0f0a1e] text-white p-4 md:p-8 font-sans selection:bg-purple-500/30">
      <div className="max-w-7xl mx-auto">
        {/* Header */}
        <header className="mb-12 flex items-center justify-between border-b border-white/10 pb-6">
          <div>
            <h1 className="text-3xl font-bold bg-gradient-to-r from-cyan-400 via-purple-400 to-amber-400 bg-clip-text text-transparent">
              HARMONIC VIBER
            </h1>
            <p className="text-white/50 font-mono mt-2 flex items-center gap-2">
              <span className="w-2 h-2 bg-green-500 rounded-full animate-pulse" />
              X1 Mainnet Connected
            </p>
          </div>
          <div className="text-right hidden md:block">
            <div className="text-xs font-mono text-cyan-400">EPOCH {Math.floor(network.epoch)}</div>
            <div className="text-xs text-purple-400">PHASE_LOCK: ACTIVE</div>
          </div>
        </header>

        {/* Top Metrics */}
        <NetworkMetrics network={network} />

        {/* Main Grid */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">

          {/* Left: Validator Identity (Passport) */}
          <div className="lg:col-span-4 h-full">
            <ValidatorIdentity vibe={myVibe} />
          </div>

          {/* Center: Phase Visualization (The Vibe) */}
          <div className="lg:col-span-4 flex flex-col items-center justify-center glass-panel rounded-xl border border-white/10 relative overflow-hidden min-h-[400px]">
            <div className="absolute top-4 left-4 text-xs font-mono text-white/30 uppercase">Local Ref Framework</div>
            <PhaseVisualizer sim={simRef.current} />
            <div className="absolute bottom-4 text-center ">
              <div className="text-xs text-white/40 mb-1">Status</div>
              <div className="text-amber-400 font-bold font-mono tracking-widest">RESONATING</div>
            </div>
          </div>

          {/* Right: Rewards (Value) */}
          <div className="lg:col-span-4 h-full">
            <RewardsPanel rewards={rewards} />

            {/* Console / Event Log */}
            <div className="mt-6">
              <EventLog events={events} />
            </div>
          </div>

        </div>
      </div>
    </div>
  )
}

export default App
