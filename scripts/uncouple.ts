/**
 * Uncouple from a Phase-Coupled Oscillator
 *
 * Remove phase coupling and return to independent oscillation.
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
const PRECISION = 1_000_000_000; // 1e9

// Seeds
const FIELD_SEED = Buffer.from("coherence_field");
const COUPLING_SEED = Buffer.from("coupling");

async function main() {
  // Parse arguments
  const args = process.argv.slice(2);

  if (args.length < 1) {
    console.log("═══════════════════════════════════════════════════════════════");
    console.log("  UNCOUPLE - Dissolve Harmonic Entanglement");
    console.log("═══════════════════════════════════════════════════════════════");
    console.log("");
    console.log("  Usage: npx tsx uncouple.ts <target_pubkey>");
    console.log("");
    console.log("  Arguments:");
    console.log("    target_pubkey - Public key of the oscillator to uncouple from");
    console.log("");
    console.log("  Example:");
    console.log("    npx tsx uncouple.ts 69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72");
    console.log("");
    console.log("═══════════════════════════════════════════════════════════════");
    return;
  }

  const targetPubkey = new PublicKey(args[0]);

  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  UNCOUPLING FROM OSCILLATOR");
  console.log("  Dissolving phase entanglement.");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("");

  // Load keypair
  const home = process.env.HOME || process.env.USERPROFILE || "";
  const keypairPath = path.join(home, ".config", "solana", "id.json");
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const sourceOscillator = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log("  Source:", sourceOscillator.publicKey.toBase58());
  console.log("  Target:", targetPubkey.toBase58());
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
  const [coupling] = PublicKey.findProgramAddressSync(
    [COUPLING_SEED, sourceOscillator.publicKey.toBuffer(), targetPubkey.toBuffer()],
    PROGRAM_ID
  );

  // Check if coupling exists
  const couplingAccount = await connection.getAccountInfo(coupling);
  if (!couplingAccount) {
    console.log("  ✗ Error: No phase coupling exists with this target");
    console.log("    You are not entangled with:", targetPubkey.toBase58());
    return;
  }

  // Decode coupling state
  try {
    const couplingState = program.coder.accounts.decode("PhaseCoupling", couplingAccount.data);
    const couplingK = couplingState.couplingK.toNumber() / PRECISION;
    const coupledAmplitude = couplingState.coupledAmplitude.toNumber();
    const couplingEpoch = couplingState.couplingEpoch.toNumber();

    console.log("  Current Coupling:");
    console.log("    Strength:       ", couplingK.toFixed(4), `(${(couplingK * 100).toFixed(1)}%)`);
    console.log("    Amplitude:      ", coupledAmplitude.toLocaleString(), "AUGMNTD");
    console.log("    Coupled Since:  Epoch", couplingEpoch);
    console.log("    Shared Emissions:", couplingState.sharedEmissions.toString());
    console.log("");
  } catch (e) {
    console.log("  ⚠ Could not decode coupling state");
  }

  // Execute uncouple
  console.log("  Uncoupling...");

  try {
    const tx = await program.methods
      .uncouple()
      .accounts({
        sourceOscillator: sourceOscillator.publicKey,
        coherenceField: coherenceField,
        targetOscillator: targetPubkey,
        coupling: coupling,
      })
      .rpc();

    console.log("");
    console.log("  ✓ UNCOUPLED SUCCESSFULLY");
    console.log("");
    console.log("  Signature:", tx);
    console.log("");
    console.log("═══════════════════════════════════════════════════════════════");
    console.log("  Phase entanglement dissolved.");
    console.log("  You are now oscillating independently.");
    console.log("  Your phase θ will no longer tend toward their phase.");
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
