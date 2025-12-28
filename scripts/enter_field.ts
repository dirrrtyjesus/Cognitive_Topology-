/**
 * Enter the Coherence Field - Become a Resonator
 */

import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, Connection } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddress } from "@solana/spl-token";
import BN from "bn.js";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const PROGRAM_ID = new PublicKey("RWcZBKQyd5rryLD8dfrMZ8imgtfqTfpGZi4gKGS8j7N");
const AUGMNTD_MINT = new PublicKey("9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W");
const RPC = "https://rpc.mainnet.x1.xyz";

// Seeds
const FIELD_SEED = Buffer.from("coherence_field");
const VAULT_SEED = Buffer.from("vault");
const RESONATOR_SEED = Buffer.from("resonator");

// Harmonic cycles
const HarmonicCycle = {
  QuarterWave: { quarterWave: {} },
  HalfWave: { halfWave: {} },
  FullWave: { fullWave: {} },
  GoldenWave: { goldenWave: {} },
};

async function main() {
  // Parse arguments
  const args = process.argv.slice(2);
  const amplitude = args[0] ? parseInt(args[0]) : 1_000_000; // Default 1M AUGMNTD
  const cycleArg = args[1] || "golden";

  const cycle = cycleArg === "quarter" ? HarmonicCycle.QuarterWave
    : cycleArg === "half" ? HarmonicCycle.HalfWave
    : cycleArg === "full" ? HarmonicCycle.FullWave
    : HarmonicCycle.GoldenWave;

  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  ENTERING THE COHERENCE FIELD");
  console.log("  You do not stake. You resonate.");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("");

  // Load keypair
  const home = process.env.HOME || process.env.USERPROFILE || "";
  const keypairPath = path.join(home, ".config", "solana", "id.json");
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const oscillator = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log("  Oscillator:", oscillator.publicKey.toBase58());
  console.log("  Amplitude: ", amplitude.toLocaleString(), "AUGMNTD");
  console.log("  Cycle:     ", cycleArg.toUpperCase(), "WAVE");
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
  const [resonator] = PublicKey.findProgramAddressSync(
    [RESONATOR_SEED, oscillator.publicKey.toBuffer()],
    PROGRAM_ID
  );

  // Get oscillator's token account
  const oscillatorTokens = await getAssociatedTokenAddress(
    AUGMNTD_MINT,
    oscillator.publicKey
  );

  console.log("  PDAs:");
  console.log("    Field:     ", coherenceField.toBase58());
  console.log("    Vault:     ", vault.toBase58());
  console.log("    Resonator: ", resonator.toBase58());
  console.log("    Tokens:    ", oscillatorTokens.toBase58());
  console.log("");

  // Check if already resonating
  const resonatorAccount = await connection.getAccountInfo(resonator);
  if (resonatorAccount) {
    console.log("  ⊙ Already resonating in the field!");

    // Decode and display resonator state
    try {
      const state = program.coder.accounts.decode("Resonator", resonatorAccount.data);
      console.log("");
      console.log("  Current State:");
      console.log("    Amplitude:  ", state.amplitude.toString());
      console.log("    Phase θ:    ", state.theta.toString());
      console.log("    τₖ:         ", state.tauK.toString());
      console.log("    Vibe State: ", Object.keys(state.vibeState)[0]);
    } catch (e) {
      // Ignore decode errors
    }
    return;
  }

  // Enter the field
  console.log("  Entering coherence field...");

  try {
    const tx = await program.methods
      .enterField(new BN(amplitude), cycle)
      .accounts({
        oscillator: oscillator.publicKey,
        coherenceField: coherenceField,
        resonator: resonator,
        oscillatorTokens: oscillatorTokens,
        vault: vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("");
    console.log("  ✓ ENTERED THE COHERENCE FIELD");
    console.log("");
    console.log("  Signature:", tx);
    console.log("");
    console.log("═══════════════════════════════════════════════════════════════");
    console.log("  You are now resonating.");
    console.log("  Phase: ATTUNING");
    console.log("  Find your frequency. The field awaits your coherence.");
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
