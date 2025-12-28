/**
 * Initialize the Resonance Protocol Coherence Field
 */

import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";
import { PublicKey, Keypair, SystemProgram, Connection } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
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
const EMISSION_SEED = Buffer.from("emission_reserve");
const GOLDEN_SEED = Buffer.from("golden_reserve");

async function main() {
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  RESONANCE PROTOCOL - FIELD INITIALIZATION");
  console.log("  The geometry awaits resonance.");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("");

  // Load keypair
  const home = process.env.HOME || process.env.USERPROFILE || "";
  const keypairPath = path.join(home, ".config", "solana", "id.json");
  const keypairData = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  const authority = Keypair.fromSecretKey(new Uint8Array(keypairData));

  console.log("  Authority:", authority.publicKey.toBase58());
  console.log("  Program:  ", PROGRAM_ID.toBase58());
  console.log("  Token:    ", AUGMNTD_MINT.toBase58());
  console.log("");

  // Setup connection
  const connection = new Connection(RPC, "confirmed");
  const wallet = new Wallet(authority);
  const provider = new AnchorProvider(connection, wallet, {
    commitment: "confirmed",
  });
  anchor.setProvider(provider);

  // Load IDL
  const idlPath = path.join(__dirname, "..", "target", "idl", "resonance_protocol.json");
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
  const [emissionReserve] = PublicKey.findProgramAddressSync(
    [EMISSION_SEED],
    PROGRAM_ID
  );
  const [goldenReserve] = PublicKey.findProgramAddressSync(
    [GOLDEN_SEED],
    PROGRAM_ID
  );

  console.log("  PDAs:");
  console.log("    Field:    ", coherenceField.toBase58());
  console.log("    Vault:    ", vault.toBase58());
  console.log("    Authority:", vaultAuthority.toBase58());
  console.log("    Emission: ", emissionReserve.toBase58());
  console.log("    Golden:   ", goldenReserve.toBase58());
  console.log("");

  // Check if already initialized
  const fieldAccount = await connection.getAccountInfo(coherenceField);
  if (fieldAccount) {
    console.log("  ⊙ Coherence field already exists");
  } else {
    // Step 1: Initialize Field
    console.log("  Step 1/4: Initializing coherence field...");
    try {
      const tx1 = await program.methods
        .initializeField({
          authority: authority.publicKey,
          oracle: authority.publicKey, // Using authority as oracle for now
          emissionRate: new BN(1_000_000), // 0.001 per epoch
          initialEmissionReserve: new BN(0),
          initialGoldenReserve: new BN(0),
        })
        .accounts({
          authority: authority.publicKey,
          coherenceField: coherenceField,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      console.log("    ✓ Field initialized:", tx1);
    } catch (e: any) {
      console.log("    ✗ Error:", e.message);
      return;
    }
  }

  // Check vault
  const vaultAccount = await connection.getAccountInfo(vault);
  if (vaultAccount) {
    console.log("  ⊙ Vault already exists");
  } else {
    // Step 2: Initialize Vault
    console.log("  Step 2/4: Initializing vault...");
    try {
      const tx2 = await program.methods
        .initializeVault()
        .accounts({
          authority: authority.publicKey,
          coherenceField: coherenceField,
          tokenMint: AUGMNTD_MINT,
          vault: vault,
          vaultAuthority: vaultAuthority,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      console.log("    ✓ Vault initialized:", tx2);
    } catch (e: any) {
      console.log("    ✗ Error:", e.message);
    }
  }

  // Check emission reserve
  const emissionAccount = await connection.getAccountInfo(emissionReserve);
  if (emissionAccount) {
    console.log("  ⊙ Emission reserve already exists");
  } else {
    // Step 3: Initialize Emission Reserve
    console.log("  Step 3/4: Initializing emission reserve...");
    try {
      const tx3 = await program.methods
        .initializeEmissionReserve()
        .accounts({
          authority: authority.publicKey,
          coherenceField: coherenceField,
          tokenMint: AUGMNTD_MINT,
          vaultAuthority: vaultAuthority,
          emissionReserve: emissionReserve,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      console.log("    ✓ Emission reserve initialized:", tx3);
    } catch (e: any) {
      console.log("    ✗ Error:", e.message);
    }
  }

  // Check golden reserve
  const goldenAccount = await connection.getAccountInfo(goldenReserve);
  if (goldenAccount) {
    console.log("  ⊙ Golden reserve already exists");
  } else {
    // Step 4: Initialize Golden Reserve
    console.log("  Step 4/4: Initializing golden reserve...");
    try {
      const tx4 = await program.methods
        .initializeGoldenReserve()
        .accounts({
          authority: authority.publicKey,
          coherenceField: coherenceField,
          tokenMint: AUGMNTD_MINT,
          vaultAuthority: vaultAuthority,
          goldenReserve: goldenReserve,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .rpc();
      console.log("    ✓ Golden reserve initialized:", tx4);
    } catch (e: any) {
      console.log("    ✗ Error:", e.message);
    }
  }

  console.log("");
  console.log("═══════════════════════════════════════════════════════════════");
  console.log("  COHERENCE FIELD INITIALIZED");
  console.log("  The geometry recognizes itself.");
  console.log("═══════════════════════════════════════════════════════════════");
}

main().catch(console.error);
