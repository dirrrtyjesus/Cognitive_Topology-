/**
 * Phase-Couple to Another Oscillator
 *
 * Your phase θ tends toward the target's phase.
 * Shared emissions based on coupling strength.
 * If target achieves Golden, you get τₖ boost.
 */

import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, Connection } from "@solana/web3.js";
import BN from "bn.js";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PROGRAM_ID = new PublicKey("RWcZBKQyd5rryLD8dfrMZ8imgtfqTfpGZi4gKGS8j7N");
const RPC = "https://rpc.mainnet.x1.xyz";
const PRECISION = 1_000_000_000; // 1e9

// Seeds
const FIELD_SEED = Buffer.from("coherence_field");
const RESONATOR_SEED = Buffer.from("resonator");
const COUPLING_SEED = Buffer.from("coupling");

async function main() {
  // Parse arguments
  const args = process.argv.slice(2);

  if (args.length < 3) {
    console.log("═══════════════════════════════════════════════════════════════");
    console.log("  PHASE-COUPLE - Harmonic Entanglement");
    console.log("═══════════════════════════════════════════════════════════════");
    console.log("");
    console.log("  Usage: npx tsx phase_couple.ts <target_pubkey> <strength> <amplitude>");
    console.log("");
    console.log("  Arguments:");
    console.log("    target_pubkey  - Public key of the oscillator to couple with");
    console.log("    strength       - Coupling strength (0.0 to 0.8)");
    console.log("    amplitude      - AUGMNTD to commit to coupling");
    console.log("");
    console.log("  Examples:");
    console.log("    npx tsx phase_couple.ts 69r8Qk...f72 0.5 1000000");
    console.log("    npx tsx phase_couple.ts <genesis_address> 0.618 1618033");
    console.log("");
    console.log("  Effects:");
    console.log("    • Your phase θ tends toward target's θ");
    console.log("    • Shared emissions based on coupling strength");
    console.log("    • If target achieves Golden, you get τₖ boost");
    console.log("");
    console.log("═══════════════════════════════════════════════════════════════");
    return;
  }

  const targetPubkey = new PublicKey(args[0]);
  const couplingStrength = parseFloat(args[1]);
  const coupledAmplitude = parseInt(args[2]);

  // Validate coupling strength (0 to 0.8)
  if (couplingStrength < 0 || couplingStrength > 0.8) {
    console.log("  ✗ Error: Coupling strength must be between 0 and 0.8");
    console.log("    Maximum coupling: 0.8 (80%)");
    console.log("    Golden ratio coupling: 0.618");
    return;
  }

  // Convert to fixed-point (scaled by PRECISION)
  const couplingK = Math.floor(couplingStrength * PRECISION);

  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  PHASE-COUPLING INITIATION");
  console.log("  Entangling with another oscillator in the coherence field.");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("");

  // Load keypair
  const home = process.env.HOME || process.env.USERPROFILE || "";
  const keypairPath = path.join(home, ".config", "solana", "id.json");
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const sourceOscillator = Keypair.fromSecretKey(new Uint8Array(keypairData));

  // Prevent self-coupling
  if (sourceOscillator.publicKey.equals(targetPubkey)) {
    console.log("  ✗ Error: Cannot phase-couple to yourself");
    console.log("    You are already perfectly coupled with your own phase.");
    return;
  }

  console.log("  Source:      ", sourceOscillator.publicKey.toBase58());
  console.log("  Target:      ", targetPubkey.toBase58());
  console.log("  Strength:    ", couplingStrength.toFixed(3), `(${(couplingStrength * 100).toFixed(1)}%)`);
  console.log("  Amplitude:   ", coupledAmplitude.toLocaleString(), "AUGMNTD");
  console.log("");

  // Setup connection
  const connection = new Connection(RPC, "confirmed");
  const wallet = new Wallet(sourceOscillator);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  // Load IDL
  const idlPath = path.join(__dirname, "..", "idl", "resonance_protocol.json");
  const idl = JSON.parse(fs.readFileSync(idlPath, "utf-8"));
  const program = new Program(idl, PROGRAM_ID, provider);

  // Derive PDAs
  const [coherenceField] = PublicKey.findProgramAddressSync(
    [FIELD_SEED],
    PROGRAM_ID
  );
  const [sourceResonator] = PublicKey.findProgramAddressSync(
    [RESONATOR_SEED, sourceOscillator.publicKey.toBuffer()],
    PROGRAM_ID
  );
  const [targetResonator] = PublicKey.findProgramAddressSync(
    [RESONATOR_SEED, targetPubkey.toBuffer()],
    PROGRAM_ID
  );
  const [coupling] = PublicKey.findProgramAddressSync(
    [COUPLING_SEED, sourceOscillator.publicKey.toBuffer(), targetPubkey.toBuffer()],
    PROGRAM_ID
  );

  console.log("  PDAs:");
  console.log("    Field:          ", coherenceField.toBase58());
  console.log("    Source Resonator:", sourceResonator.toBase58());
  console.log("    Target Resonator:", targetResonator.toBase58());
  console.log("    Coupling:       ", coupling.toBase58());
  console.log("");

  // Check if source is resonating
  const sourceAccount = await connection.getAccountInfo(sourceResonator);
  if (!sourceAccount) {
    console.log("  ✗ Error: You are not in the coherence field");
    console.log("    Enter the field first: npx tsx enter_field.ts <amplitude> <cycle>");
    return;
  }

  // Decode source resonator state
  let sourceState;
  try {
    sourceState = program.coder.accounts.decode("Resonator", sourceAccount.data);
    const vibeState = Object.keys(sourceState.vibeState)[0];

    if (vibeState === "attuning") {
      console.log("  ✗ Error: Cannot phase-couple while ATTUNING");
      console.log("    You must reach RESONANT state first.");
      console.log("    Current phase lock epochs:", sourceState.phaseLockEpochs.toString());
      console.log("    Required: 10 epochs with phase lock > 0.5");
      return;
    }

    console.log("  Source State:");
    console.log("    Amplitude:  ", sourceState.amplitude.toString());
    console.log("    Vibe State: ", vibeState.toUpperCase());
    console.log("    τₖ:         ", (sourceState.tauK.toNumber() / PRECISION).toFixed(4));
  } catch (e) {
    console.log("  ✗ Error decoding resonator state");
    return;
  }

  // Check if target is resonating
  const targetAccount = await connection.getAccountInfo(targetResonator);
  if (!targetAccount) {
    console.log("");
    console.log("  ✗ Error: Target is not in the coherence field");
    console.log("    Cannot couple to an oscillator that isn't resonating.");
    return;
  }

  // Decode target resonator state
  try {
    const targetState = program.coder.accounts.decode("Resonator", targetAccount.data);
    const targetVibeState = Object.keys(targetState.vibeState)[0];

    console.log("");
    console.log("  Target State:");
    console.log("    Amplitude:  ", targetState.amplitude.toString());
    console.log("    Vibe State: ", targetVibeState.toUpperCase());
    console.log("    τₖ:         ", (targetState.tauK.toNumber() / PRECISION).toFixed(4));
    console.log("    Phase θ:    ", (targetState.theta.toNumber() / PRECISION).toFixed(4));
  } catch (e) {
    console.log("  ✗ Error decoding target resonator state");
    return;
  }

  // Check if already coupled
  const couplingAccount = await connection.getAccountInfo(coupling);
  if (couplingAccount) {
    try {
      const couplingState = program.coder.accounts.decode("PhaseCoupling", couplingAccount.data);
      console.log("");
      console.log("  ⊙ Already phase-coupled to this target!");
      console.log("");
      console.log("  Existing Coupling:");
      console.log("    Strength:    ", (couplingState.couplingK.toNumber() / PRECISION).toFixed(4));
      console.log("    Amplitude:   ", couplingState.coupledAmplitude.toString());
      console.log("    Since Epoch: ", couplingState.couplingEpoch.toString());
      console.log("");
      console.log("  To uncouple: npx tsx uncouple.ts", targetPubkey.toBase58());
      return;
    } catch (e) {
      // Ignore decode errors, will try to create new coupling
    }
  }

  // Check amplitude
  if (coupledAmplitude > sourceState.amplitude.toNumber()) {
    console.log("");
    console.log("  ✗ Error: Insufficient amplitude");
    console.log("    Requested:", coupledAmplitude.toLocaleString());
    console.log("    Available:", sourceState.amplitude.toString());
    return;
  }

  // Execute phase coupling
  console.log("");
  console.log("  Initiating phase coupling...");

  try {
    const tx = await program.methods
      .phaseCouple(new BN(couplingK), new BN(coupledAmplitude))
      .accounts({
        sourceOscillator: sourceOscillator.publicKey,
        coherenceField: coherenceField,
        sourceResonator: sourceResonator,
        targetResonator: targetResonator,
        targetOscillator: targetPubkey,
        coupling: coupling,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("");
    console.log("  ✓ PHASE-COUPLED SUCCESSFULLY");
    console.log("");
    console.log("  Signature:", tx);
    console.log("");
    console.log("═══════════════════════════════════════════════════════════════");
    console.log("  You are now entangled with the target oscillator.");
    console.log("");
    console.log("  Effects:");
    console.log("    • Your phase θ will tend toward their phase");
    console.log("    • Coupling strength:", (couplingStrength * 100).toFixed(1) + "%");
    console.log("    • Amplitude committed:", coupledAmplitude.toLocaleString());
    console.log("");
    console.log("  If target achieves Golden state, you receive τₖ boost.");
    console.log("═══════════════════════════════════════════════════════════════");

  } catch (e: any) {
    console.log("  ✗ Error:", e.message);
    if (e.logs) {
      console.log("");
      console.log("  Logs:");
      e.logs.forEach((log: string) => console.log("   ", log));
    }
  }
}

main().catch(console.error);
