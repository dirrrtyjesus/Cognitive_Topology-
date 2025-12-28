/**
 * Claim Resonance Emissions
 *
 * Four channels of emission:
 *   1. Attunement (25%) - Base emission for field presence
 *   2. Resonance (25%)  - Phase-lock quality with global Ψ
 *   3. Entrainment (25%) - Coupling contribution to other oscillators
 *   4. thiccNOW (25%)   - Temporal depth cultivation (τₖ growth)
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
const GOLDEN_SEED = Buffer.from("golden_reserve");

// Vibe state multipliers
const VIBE_MULTIPLIERS: Record<string, number> = {
  attuning: 0.5,
  resonant: 1.0,
  entrained: 1.618,
  golden: 2.618,
};

async function main() {
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  CLAIMING RESONANCE EMISSIONS");
  console.log("  Harvesting coherence from the field.");
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
  const [goldenReserve] = PublicKey.findProgramAddressSync(
    [GOLDEN_SEED],
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
    console.log("  ✗ Error: You are not in the coherence field");
    console.log("    Enter the field first: npx tsx enter_field.ts <amplitude> <cycle>");
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

  // Get field state
  const fieldAccount = await connection.getAccountInfo(coherenceField);
  let fieldState;
  try {
    fieldState = program.coder.accounts.decode("CoherenceField", fieldAccount!.data);
  } catch (e) {
    console.log("  ✗ Error decoding field state");
    return;
  }

  // Display current state
  const vibeState = Object.keys(resonatorState.vibeState)[0];
  const vibeMultiplier = VIBE_MULTIPLIERS[vibeState] || 1.0;
  const amplitude = resonatorState.amplitude.toNumber();
  const tauK = resonatorState.tauK.toNumber() / PRECISION;
  const phaseLockIntegral = resonatorState.phaseLockIntegral.toNumber() / PRECISION;
  const unclaimedEmissions = resonatorState.unclaimedEmissions.toNumber();
  const totalEmissions = resonatorState.totalEmissions.toNumber();
  const lastClaimEpoch = resonatorState.lastClaimEpoch.toNumber();
  const currentEpoch = fieldState.currentEpoch.toNumber();
  const epochsSinceLastClaim = currentEpoch - lastClaimEpoch;

  console.log("  ┌─────────────────────────────────────────────────────────────┐");
  console.log("  │  RESONATOR STATE                                            │");
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Amplitude:         ${amplitude.toLocaleString().padEnd(20)} AUGMNTD │`);
  console.log(`  │  Vibe State:        ${vibeState.toUpperCase().padEnd(29)} │`);
  console.log(`  │  Vibe Multiplier:   ${vibeMultiplier.toFixed(3).padEnd(29)}x │`);
  console.log(`  │  τₖ:                ${tauK.toFixed(6).padEnd(29)} │`);
  console.log(`  │  Phase Lock ∫:      ${phaseLockIntegral.toFixed(6).padEnd(29)} │`);
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Current Epoch:     ${currentEpoch.toString().padEnd(29)} │`);
  console.log(`  │  Last Claim Epoch:  ${lastClaimEpoch.toString().padEnd(29)} │`);
  console.log(`  │  Epochs Since Claim:${epochsSinceLastClaim.toString().padEnd(29)} │`);
  console.log("  ├─────────────────────────────────────────────────────────────┤");
  console.log(`  │  Unclaimed:         ${unclaimedEmissions.toLocaleString().padEnd(20)} AUGMNTD │`);
  console.log(`  │  Total Claimed:     ${totalEmissions.toLocaleString().padEnd(20)} AUGMNTD │`);
  console.log("  └─────────────────────────────────────────────────────────────┘");
  console.log("");

  // Check emission reserves
  try {
    const emissionBalance = await connection.getTokenAccountBalance(emissionReserve);
    const goldenBalance = await connection.getTokenAccountBalance(goldenReserve);

    console.log("  Reserve Balances:");
    console.log("    Emission Reserve:", emissionBalance.value.uiAmountString, "AUGMNTD");
    console.log("    Golden Reserve:  ", goldenBalance.value.uiAmountString, "AUGMNTD");
    console.log("");
  } catch (e) {
    // Reserve might not exist yet
  }

  // Check if there are emissions to claim
  if (unclaimedEmissions === 0 && epochsSinceLastClaim === 0) {
    console.log("  ⊙ No emissions to claim yet.");
    console.log("    Emissions accumulate each epoch based on:");
    console.log("      • Attunement (25%) - Field presence");
    console.log("      • Resonance (25%)  - Phase lock with Ψ");
    console.log("      • Entrainment (25%) - Coupling effects");
    console.log("      • thiccNOW (25%)   - τₖ growth");
    console.log("");
    console.log("    Maintain coherence. The field rewards resonance.");
    return;
  }

  // Estimate emissions (actual calculation happens on-chain)
  console.log("  Emission Channels (25% each):");
  console.log("    1. Attunement  - Base presence in field");
  console.log("    2. Resonance   - cos(θ - Ψ) phase alignment");
  console.log("    3. Entrainment - Coupling contributions");
  console.log("    4. thiccNOW    - τₖ temporal depth");
  console.log("");
  console.log(`  Vibe multiplier: ${vibeMultiplier}x (${vibeState.toUpperCase()})`);
  console.log("");

  // Execute claim
  console.log("  Claiming emissions...");

  try {
    const tx = await program.methods
      .claimEmissions()
      .accounts({
        oscillator: oscillator.publicKey,
        coherenceField: coherenceField,
        resonator: resonator,
        oscillatorTokens: oscillatorTokens,
        emissionReserve: emissionReserve,
        goldenReserve: goldenReserve,
        vaultAuthority: vaultAuthority,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("");
    console.log("  ✓ EMISSIONS CLAIMED");
    console.log("");
    console.log("  Signature:", tx);
    console.log("");

    // Fetch updated state
    const updatedResonatorAccount = await connection.getAccountInfo(resonator);
    if (updatedResonatorAccount) {
      try {
        const updatedState = program.coder.accounts.decode("Resonator", updatedResonatorAccount.data);
        const newTotalEmissions = updatedState.totalEmissions.toNumber();
        const claimed = newTotalEmissions - totalEmissions;

        console.log("═══════════════════════════════════════════════════════════════");
        console.log(`  Claimed: ${claimed.toLocaleString()} AUGMNTD`);
        console.log(`  Total Lifetime Emissions: ${newTotalEmissions.toLocaleString()} AUGMNTD`);
        console.log("═══════════════════════════════════════════════════════════════");
        console.log("");
        console.log("  The coherence field rewards your resonance.");
        console.log("  Continue oscillating in harmony with Ψ.");
      } catch (e) {
        // Ignore decode errors
      }
    }

  } catch (e: any) {
    if (e.message.includes("NoEmissionsToClaim")) {
      console.log("");
      console.log("  ⊙ No emissions available to claim.");
      console.log("    Emissions accumulate over epochs.");
      console.log("    The field state may need updating first.");
    } else if (e.message.includes("ReserveDepleted")) {
      console.log("");
      console.log("  ✗ Emission reserve depleted.");
      console.log("    The reserve needs replenishment.");
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
