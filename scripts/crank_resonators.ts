/**
 * Crank Resonator Phase Updates
 *
 * Anyone can call this to update resonator phases.
 * Applies Kuramoto dynamics: dθᵢ = ωᵢ + K·R·sin(Ψ - θᵢ)
 *
 * This is permissionless - run it to help the network stay synchronized.
 */

import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PROGRAM_ID = new PublicKey("RWcZBKQyd5rryLD8dfrMZ8imgtfqTfpGZi4gKGS8j7N");
const RPC = "https://rpc.mainnet.x1.xyz";
const PRECISION = 1_000_000_000;

// Seeds
const FIELD_SEED = Buffer.from("coherence_field");
const RESONATOR_SEED = Buffer.from("resonator");

// Known resonators (add more as they enter the field)
const KNOWN_RESONATORS = [
  "69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72", // Genesis resonator
];

async function main() {
  const args = process.argv.slice(2);

  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  CRANK RESONATOR PHASES");
  console.log("  Applying Kuramoto dynamics to all resonators.");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("");

  // Load keypair (anyone can crank)
  const home = process.env.HOME || process.env.USERPROFILE || "";
  const keypairPath = path.join(home, ".config", "solana", "id.json");
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const crank = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log("  Crank:", crank.publicKey.toBase58());
  console.log("");

  // Setup connection
  const connection = new Connection(RPC, "confirmed");
  const wallet = new Wallet(crank);
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

  const currentEpoch = fieldState.currentEpoch.toNumber();
  const globalPsi = fieldState.globalPsi.toNumber();
  const orderParameter = fieldState.orderParameter.toNumber();

  console.log("  Field State:");
  console.log("    Epoch:    ", currentEpoch);
  console.log("    Global Ψ: ", (globalPsi / PRECISION).toFixed(6));
  console.log("    Order R:  ", (orderParameter / PRECISION).toFixed(6));
  console.log("");

  // Collect resonators to crank
  let resonatorsToCrank: string[] = [];

  // If specific resonators provided as args
  if (args.length > 0 && !args[0].startsWith("-")) {
    resonatorsToCrank = args.filter(a => !a.startsWith("-"));
  } else {
    // Use known resonators list
    resonatorsToCrank = [...KNOWN_RESONATORS];

    // Also add the crank's own address if they're resonating
    if (!resonatorsToCrank.includes(crank.publicKey.toBase58())) {
      resonatorsToCrank.push(crank.publicKey.toBase58());
    }
  }

  console.log("  Cranking", resonatorsToCrank.length, "resonator(s)...");
  console.log("");

  let updated = 0;
  let skipped = 0;
  let failed = 0;

  for (const oscillatorAddr of resonatorsToCrank) {
    let oscillatorPubkey: PublicKey;
    try {
      oscillatorPubkey = new PublicKey(oscillatorAddr);
    } catch (e) {
      console.log("    ⚠ Invalid pubkey:", oscillatorAddr);
      skipped++;
      continue;
    }

    const [resonatorPda] = PublicKey.findProgramAddressSync(
      [RESONATOR_SEED, oscillatorPubkey.toBuffer()],
      PROGRAM_ID
    );

    // Check if resonator exists
    const resonatorAccount = await connection.getAccountInfo(resonatorPda);
    if (!resonatorAccount) {
      console.log("    ⊙ Not resonating:", oscillatorAddr.slice(0, 8) + "...");
      skipped++;
      continue;
    }

    // Decode current state
    let resonatorState;
    try {
      resonatorState = program.coder.accounts.decode("Resonator", resonatorAccount.data);
    } catch (e) {
      console.log("    ⚠ Could not decode:", oscillatorAddr.slice(0, 8) + "...");
      skipped++;
      continue;
    }

    const oldTheta = resonatorState.theta.toNumber();
    const vibeState = Object.keys(resonatorState.vibeState)[0];

    // Execute phase update
    try {
      const tx = await program.methods
        .updateResonatorPhase()
        .accounts({
          crank: crank.publicKey,
          coherenceField: coherenceField,
          resonator: resonatorPda,
        })
        .rpc();

      // Fetch new state
      const newResonatorAccount = await connection.getAccountInfo(resonatorPda);
      let newTheta = oldTheta;
      let phaseLock = 0;

      if (newResonatorAccount) {
        try {
          const newState = program.coder.accounts.decode("Resonator", newResonatorAccount.data);
          newTheta = newState.theta.toNumber();
          // Calculate phase lock
          const diff = Math.abs(newTheta - globalPsi);
          phaseLock = Math.cos(diff / PRECISION);
        } catch (e) {
          // Ignore
        }
      }

      console.log(`    ✓ ${oscillatorAddr.slice(0, 8)}... [${vibeState.toUpperCase()}]`);
      console.log(`      θ: ${(oldTheta / PRECISION).toFixed(4)} → ${(newTheta / PRECISION).toFixed(4)}`);
      console.log(`      Phase lock: ${(phaseLock * 100).toFixed(1)}%`);
      updated++;

    } catch (e: any) {
      console.log(`    ✗ ${oscillatorAddr.slice(0, 8)}...: ${e.message}`);
      failed++;
    }
  }

  console.log("");
  console.log("  ─────────────────────────────────────────────────────────────");
  console.log("  Summary:");
  console.log("    Updated:", updated);
  console.log("    Skipped:", skipped);
  console.log("    Failed: ", failed);
  console.log("");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  Kuramoto dynamics applied.");
  console.log("  Resonators are synchronizing with the global phase Ψ.");
  console.log("═══════════════════════════════════════════════════════════════");
}

main().catch(console.error);
