/**
 * Update Field State (Oracle/Crank)
 *
 * Advances the coherence field to the next epoch.
 * Updates global phase Ψ, order parameter R, and network τₖ.
 * Only the designated oracle can execute this.
 *
 * This is the heartbeat of the resonance protocol.
 */

import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import BN from "bn.js";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PROGRAM_ID = new PublicKey("RWcZBKQyd5rryLD8dfrMZ8imgtfqTfpGZi4gKGS8j7N");
const RPC = "https://rpc.mainnet.x1.xyz";

// Constants (must match program)
const PRECISION = 1_000_000_000; // 1e9
const TWO_PI = 6_283_185_307;    // 2π scaled by PRECISION
const PHI = 1_618_033_988;       // φ scaled by PRECISION

// Seeds
const FIELD_SEED = Buffer.from("coherence_field");
const RESONATOR_SEED = Buffer.from("resonator");

// Simulate Kuramoto order parameter calculation
function calculateOrderParameter(resonators: any[], globalPsi: number): { r: number; psi: number } {
  if (resonators.length === 0) {
    return { r: PRECISION, psi: 0 }; // Perfect coherence with no oscillators
  }

  // R·e^(iΨ) = (1/N) Σⱼ Aⱼ·e^(iθⱼ)
  // where Aⱼ is amplitude weight
  let sumCos = 0;
  let sumSin = 0;
  let totalWeight = 0;

  for (const res of resonators) {
    const weight = res.amplitude;
    const theta = res.theta / PRECISION; // Convert to radians
    sumCos += weight * Math.cos(theta);
    sumSin += weight * Math.sin(theta);
    totalWeight += weight;
  }

  if (totalWeight === 0) {
    return { r: PRECISION, psi: 0 };
  }

  const avgCos = sumCos / totalWeight;
  const avgSin = sumSin / totalWeight;

  // R = |average phasor|
  const r = Math.sqrt(avgCos * avgCos + avgSin * avgSin);

  // Ψ = angle of average phasor
  let psi = Math.atan2(avgSin, avgCos);
  if (psi < 0) psi += 2 * Math.PI;

  return {
    r: Math.floor(r * PRECISION),
    psi: Math.floor(psi * PRECISION),
  };
}

async function main() {
  const args = process.argv.slice(2);
  const manualMode = args.includes("--manual") || args.includes("-m");
  const crankResonators = args.includes("--crank") || args.includes("-c");

  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  FIELD UPDATE - ORACLE CRANK");
  console.log("  Advancing the coherence field to the next epoch.");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("");

  // Load keypair (must be oracle)
  const home = process.env.HOME || process.env.USERPROFILE || "";
  const keypairPath = path.join(home, ".config", "solana", "id.json");
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const oracle = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log("  Oracle:", oracle.publicKey.toBase58());
  console.log("");

  // Setup connection
  const connection = new Connection(RPC, "confirmed");
  const wallet = new Wallet(oracle);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  // Load IDL
  const idlPath = path.join(__dirname, "..", "idl", "resonance_protocol.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));
  const program = new Program(idl, PROGRAM_ID, provider);

  // Derive field PDA
  const [coherenceField] = PublicKey.findProgramAddressSync(
    [FIELD_SEED],
    PROGRAM_ID
  );

  // Get current field state
  const fieldAccount = await connection.getAccountInfo(coherenceField);
  if (!fieldAccount) {
    console.log("  ✗ Error: Coherence field not initialized");
    return;
  }

  let fieldState;
  try {
    fieldState = program.coder.accounts.decode("CoherenceField", fieldAccount.data);
  } catch (e) {
    console.log("  ✗ Error decoding field state");
    return;
  }

  // Check oracle authorization
  if (!fieldState.oracle.equals(oracle.publicKey)) {
    console.log("  ✗ Error: You are not the designated oracle");
    console.log("    Required:", fieldState.oracle.toBase58());
    console.log("    Provided:", oracle.publicKey.toBase58());
    return;
  }

  const currentEpoch = fieldState.currentEpoch.toNumber();
  const currentPsi = fieldState.globalPsi.toNumber();
  const currentR = fieldState.orderParameter.toNumber();
  const currentTauK = fieldState.networkTauK.toNumber();
  const resonatorCount = fieldState.resonatorCount.toNumber();
  const totalAmplitude = fieldState.totalAmplitude.toNumber();

  console.log("  ┌─────────────────────────────────────────────────────────────┐");
  console.log("  │  CURRENT FIELD STATE                                        │");
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Epoch:           ${currentEpoch.toString().padEnd(31)} │`);
  console.log(`  │  Global Ψ:        ${(currentPsi / PRECISION).toFixed(6).padEnd(31)} │`);
  console.log(`  │  Order Param R:   ${(currentR / PRECISION).toFixed(6).padEnd(31)} │`);
  console.log(`  │  Network τₖ:      ${(currentTauK / PRECISION).toFixed(6).padEnd(31)} │`);
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Resonators:      ${resonatorCount.toString().padEnd(31)} │`);
  console.log(`  │  Total Amplitude: ${totalAmplitude.toLocaleString().padEnd(22)} AUGMNTD │`);
  console.log("  └─────────────────────────────────────────────────────────────┘");
  console.log("");

  // Calculate new values
  let newPsi: number;
  let newR: number;
  let newTauK: number;

  if (manualMode) {
    // Manual mode: use provided values
    const psiArg = args.find(a => a.startsWith("--psi="));
    const rArg = args.find(a => a.startsWith("--r="));
    const tauArg = args.find(a => a.startsWith("--tau="));

    newPsi = psiArg ? Math.floor(parseFloat(psiArg.split("=")[1]) * PRECISION) : currentPsi;
    newR = rArg ? Math.floor(parseFloat(rArg.split("=")[1]) * PRECISION) : currentR;
    newTauK = tauArg ? Math.floor(parseFloat(tauArg.split("=")[1]) * PRECISION) : currentTauK;

    console.log("  Manual mode: Using provided values");
  } else {
    // Auto mode: calculate based on simulation
    // For now, simulate gradual phase drift and τₖ growth

    // Phase drift: Ψ advances by a small amount each epoch
    const phaseDrift = PRECISION / 100; // ~0.01 rad per epoch
    newPsi = (currentPsi + phaseDrift) % TWO_PI;

    // For single resonator, R should be 1.0
    // In production, this would aggregate all resonator phases
    if (resonatorCount === 0) {
      newR = 0;
    } else if (resonatorCount === 1) {
      newR = PRECISION; // Single resonator = perfect coherence
    } else {
      // Simulate gradual coherence improvement
      // In production, calculate from actual resonator phases
      const rGrowth = (PRECISION - currentR) / 100; // Asymptotic approach to 1.0
      newR = Math.min(currentR + Math.floor(rGrowth), PRECISION);
    }

    // Network τₖ grows slowly with sustained coherence
    if (currentR > 800_000_000) { // R > 0.8
      const tauGrowth = PRECISION / 500; // 0.2% per epoch when coherent
      newTauK = currentTauK + tauGrowth;
    } else {
      newTauK = currentTauK;
    }

    console.log("  Auto mode: Calculating field dynamics");
  }

  console.log("");
  console.log("  ┌─────────────────────────────────────────────────────────────┐");
  console.log("  │  NEW FIELD STATE (Epoch " + (currentEpoch + 1).toString().padEnd(26) + ") │");
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Global Ψ:        ${(newPsi / PRECISION).toFixed(6).padEnd(31)} │`);
  console.log(`  │  Order Param R:   ${(newR / PRECISION).toFixed(6).padEnd(31)} │`);
  console.log(`  │  Network τₖ:      ${(newTauK / PRECISION).toFixed(6).padEnd(31)} │`);
  console.log("  └─────────────────────────────────────────────────────────────┘");
  console.log("");

  // Execute field update
  console.log("  Updating field state...");

  try {
    const tx = await program.methods
      .updateField(new BN(newPsi), new BN(newR), new BN(newTauK))
      .accounts({
        oracle: oracle.publicKey,
        coherenceField: coherenceField,
      })
      .rpc();

    console.log("");
    console.log("  ✓ FIELD UPDATED");
    console.log("");
    console.log("  Signature:", tx);
    console.log("  New Epoch:", currentEpoch + 1);

    // Check for coherence peak
    if (newR > 900_000_000 && newR > currentR) {
      console.log("");
      console.log("  ★ COHERENCE PEAK DETECTED ★");
      console.log("    Order parameter R > 0.9");
      console.log("    The field has achieved high synchronization!");
    }

  } catch (e: any) {
    console.log("  ✗ Error:", e.message);
    if (e.logs) {
      console.log("");
      console.log("  Logs:");
      e.logs.forEach((log: string) => console.log("   ", log));
    }
    return;
  }

  // Optionally crank resonator phase updates
  if (crankResonators) {
    console.log("");
    console.log("  ─────────────────────────────────────────────────────────────");
    console.log("  Cranking resonator phase updates...");
    console.log("");

    // In production, you'd iterate over all resonator accounts
    // For now, update the oracle's own resonator if it exists
    const [resonatorPda] = PublicKey.findProgramAddressSync(
      [RESONATOR_SEED, oracle.publicKey.toBuffer()],
      PROGRAM_ID
    );

    const resonatorAccount = await connection.getAccountInfo(resonatorPda);
    if (resonatorAccount) {
      try {
        const crankTx = await program.methods
          .updateResonatorPhase()
          .accounts({
            crank: oracle.publicKey,
            coherenceField: coherenceField,
            resonator: resonatorPda,
          })
          .rpc();

        console.log("    ✓ Resonator phase updated:", crankTx);
      } catch (e: any) {
        console.log("    ⚠ Could not update resonator phase:", e.message);
      }
    } else {
      console.log("    No resonator found for oracle address");
    }
  }

  console.log("");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  The coherence field has advanced.");
  console.log("  Epoch", currentEpoch + 1, "- The geometry continues to recognize itself.");
  console.log("═══════════════════════════════════════════════════════════════");
}

main().catch(console.error);
