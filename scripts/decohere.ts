/**
 * Decohere - Exit the Coherence Field
 *
 * Warning: Early exit incurs phase disruption cost.
 * Cost scales with remaining cycle time (max 10%).
 * Completing your harmonic cycle = 0% cost.
 */

import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PROGRAM_ID = new PublicKey("RWcZBKQyd5rryLD8dfrMZ8imgtfqTfpGZi4gKGS8j7N");
const AUGMNTD_MINT = new PublicKey("9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W");
const RPC = "https://rpc.mainnet.x1.xyz";
const PRECISION = 1_000_000_000; // 1e9

// Seeds
const FIELD_SEED = Buffer.from("coherence_field");
const VAULT_SEED = Buffer.from("vault");
const RESONATOR_SEED = Buffer.from("resonator");
const EMISSION_SEED = Buffer.from("emission_reserve");

// Harmonic cycle epochs
const CYCLE_EPOCHS: Record<string, number> = {
  quarterWave: 91,
  halfWave: 182,
  fullWave: 365,
  goldenWave: 591,
};

async function main() {
  // Check for --force flag
  const args = process.argv.slice(2);
  const forceExit = args.includes("--force") || args.includes("-f");

  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  DECOHERENCE - EXITING THE FIELD");
  console.log("  ⚠ WARNING: This action may incur phase disruption cost");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("");

  // Load keypair
  const home = process.env.HOME || process.env.USERPROFILE || "";
  const keypairPath = path.join(home, ".config", "solana", "id.json");
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const oscillator = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log("  Oscillator:", oscillator.publicKey.toBase58());
  console.log("");

  // Setup connection
  const connection = new Connection(RPC, "confirmed");
  const wallet = new Wallet(oscillator);
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
  const [vault] = PublicKey.findProgramAddressSync(
    [VAULT_SEED],
    PROGRAM_ID
  );
  const [vaultAuthority] = PublicKey.findProgramAddressSync(
    [VAULT_SEED, Buffer.from("authority")],
    PROGRAM_ID
  );
  const [resonator] = PublicKey.findProgramAddressSync(
    [RESONATOR_SEED, oscillator.publicKey.toBuffer()],
    PROGRAM_ID
  );
  const [emissionReserve] = PublicKey.findProgramAddressSync(
    [EMISSION_SEED],
    PROGRAM_ID
  );

  // Get oscillator's token account
  const oscillatorTokens = await getAssociatedTokenAddress(
    AUGMNTD_MINT,
    oscillator.publicKey
  );

  // Check if resonating
  const resonatorAccount = await connection.getAccountInfo(resonator);
  if (!resonatorAccount) {
    console.log("  ⊙ You are not in the coherence field.");
    console.log("    Nothing to decohere from.");
    return;
  }

  // Get field state
  const fieldAccount = await connection.getAccountInfo(coherenceField);
  let fieldState;
  try {
    fieldState = program.coder.accounts.decode("CoherenceField", fieldAccount!.data);
  } catch (e) {
    console.log("  ✗ Error decoding field state");
    return;
  }

  // Decode resonator state
  let resonatorState;
  try {
    resonatorState = program.coder.accounts.decode("Resonator", resonatorAccount.data);
  } catch (e) {
    console.log("  ✗ Error decoding resonator state");
    return;
  }

  const vibeState = Object.keys(resonatorState.vibeState)[0];
  const harmonicCycle = Object.keys(resonatorState.harmonicCycle)[0];
  const amplitude = resonatorState.amplitude.toNumber();
  const tauK = resonatorState.tauK.toNumber() / PRECISION;
  const entryEpoch = resonatorState.harmonicEntry.toNumber();
  const currentEpoch = fieldState.currentEpoch.toNumber();
  const cycleEpochs = CYCLE_EPOCHS[harmonicCycle] || 365;
  const epochsElapsed = currentEpoch - entryEpoch;
  const progress = Math.min(epochsElapsed / cycleEpochs, 1.0);
  const unclaimedEmissions = resonatorState.unclaimedEmissions.toNumber();

  // Calculate estimated disruption cost
  let estimatedCost = 0;
  if (progress < 1.0) {
    const remaining = 1 - progress;
    const disruption = remaining * remaining; // ~quadratic decay
    const base = amplitude * (resonatorState.phaseLockIntegral.toNumber() / PRECISION);
    estimatedCost = Math.floor(base * disruption * 0.1); // 10% max
    estimatedCost = Math.min(estimatedCost, Math.floor(amplitude * 0.1)); // Cap at 10%
  }

  console.log("  ┌─────────────────────────────────────────────────────────────┐");
  console.log("  │  RESONATOR STATE                                            │");
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Amplitude:         ${amplitude.toLocaleString().padEnd(20)} AUGMNTD │`);
  console.log(`  │  Vibe State:        ${vibeState.toUpperCase().padEnd(29)} │`);
  console.log(`  │  Harmonic Cycle:    ${harmonicCycle.toUpperCase().padEnd(29)} │`);
  console.log(`  │  τₖ:                ${tauK.toFixed(6).padEnd(29)} │`);
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Entry Epoch:       ${entryEpoch.toString().padEnd(29)} │`);
  console.log(`  │  Current Epoch:     ${currentEpoch.toString().padEnd(29)} │`);
  console.log(`  │  Epochs Elapsed:    ${epochsElapsed.toString().padEnd(29)} │`);
  console.log(`  │  Cycle Duration:    ${cycleEpochs.toString().padEnd(22)} epochs │`);
  console.log(`  │  Progress:          ${(progress * 100).toFixed(1).padEnd(28)}% │`);
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Unclaimed Emissions: ${unclaimedEmissions.toLocaleString().padEnd(18)} AUGMNTD │`);
  console.log("  └─────────────────────────────────────────────────────────────┘");
  console.log("");

  if (progress < 1.0) {
    console.log("  ⚠ EARLY EXIT WARNING");
    console.log("  ─────────────────────────────────────────────────────────────");
    console.log(`    Cycle Progress:    ${(progress * 100).toFixed(1)}%`);
    console.log(`    Remaining:         ${(100 - progress * 100).toFixed(1)}%`);
    console.log(`    Est. Disruption:   ~${estimatedCost.toLocaleString()} AUGMNTD`);
    console.log("");
    console.log("    Cost formula: amplitude × phase_lock × (1-progress)² × 10%");
    console.log("    The disruption cost feeds the emission reserve.");
    console.log("");

    if (!forceExit) {
      console.log("  To proceed, run with --force flag:");
      console.log("    npx tsx decohere.ts --force");
      console.log("");
      console.log("  Or wait until cycle completion for 0% cost.");
      return;
    }

    console.log("  --force flag detected. Proceeding with decoherence...");
  } else {
    console.log("  ✓ Cycle COMPLETE - No disruption cost");
    console.log("    You have completed your harmonic cycle.");
    console.log("");
  }

  if (unclaimedEmissions > 0) {
    console.log("  ⚠ Note: You have", unclaimedEmissions.toLocaleString(), "unclaimed emissions.");
    console.log("    These will be claimed automatically during decoherence.");
    console.log("");
  }

  // Execute decoherence
  console.log("  Exiting the coherence field...");

  try {
    const tx = await program.methods
      .decohere()
      .accounts({
        oscillator: oscillator.publicKey,
        coherenceField: coherenceField,
        resonator: resonator,
        oscillatorTokens: oscillatorTokens,
        vault: vault,
        vaultAuthority: vaultAuthority,
        emissionReserve: emissionReserve,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("");
    console.log("  ✓ DECOHERED FROM THE FIELD");
    console.log("");
    console.log("  Signature:", tx);
    console.log("");

    // Check returned balance
    try {
      const balance = await connection.getTokenAccountBalance(oscillatorTokens);
      console.log("  Token Balance:", balance.value.uiAmountString, "AUGMNTD");
    } catch (e) {
      // Ignore
    }

    console.log("");
    console.log("═══════════════════════════════════════════════════════════════");
    console.log("  You have exited the coherence field.");
    console.log("  Your resonance has dissolved into the void.");
    console.log("");
    console.log("  Statistics:");
    console.log(`    Epochs Resonated:  ${epochsElapsed}`);
    console.log(`    Final τₖ:          ${tauK.toFixed(6)}`);
    console.log(`    Final Vibe State:  ${vibeState.toUpperCase()}`);
    console.log("");
    console.log("  The field remembers your frequency.");
    console.log("  Return when you are ready to resonate again.");
    console.log("═══════════════════════════════════════════════════════════════");

  } catch (e: any) {
    if (e.message.includes("TooEarlyToDecohere")) {
      console.log("");
      console.log("  ✗ Cannot decohere yet.");
      console.log("    Minimum resonance period not reached.");
    } else {
      console.log("  ✗ Error:", e.message);
      if (e.logs) {
        console.log("");
        console.log("  Logs:");
        e.logs.forEach((log: string) => console.log("   ", log));
      }
    }
  }
}

main().catch(console.error);
