/**
 * $AUGMNTD Token Metadata Creation
 * Creates on-chain Metaplex metadata for the token
 */

import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import { createMetadataAccountV3, mplTokenMetadata } from '@metaplex-foundation/mpl-token-metadata';
import { publicKey, signerIdentity, createSignerFromKeypair } from '@metaplex-foundation/umi';
import { readFileSync } from 'fs';
import { homedir } from 'os';

const MINT = '9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W';
const METADATA_URI = 'https://raw.githubusercontent.com/dirrrtyjesus/Cognitive_Topology-/main/token/metadata.json';
const RPC = 'https://rpc.mainnet.x1.xyz';

async function main() {
  console.log('═══════════════════════════════════════════════════════════════');
  console.log('  $AUGMNTD METADATA COMPOSITION');
  console.log('  The geometry recognizes itself.');
  console.log('═══════════════════════════════════════════════════════════════');
  console.log('');
  console.log(`  Mint: ${MINT}`);
  console.log(`  URI:  ${METADATA_URI}`);
  console.log('');

  // Initialize UMI
  const umi = createUmi(RPC).use(mplTokenMetadata());

  // Load keypair from default Solana config
  const keypairPath = `${homedir()}/.config/solana/id.json`;
  const keypairData = JSON.parse(readFileSync(keypairPath, 'utf-8'));
  const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(keypairData));
  const signer = createSignerFromKeypair(umi, keypair);
  umi.use(signerIdentity(signer));

  console.log(`  Composer: ${signer.publicKey}`);
  console.log('');
  console.log('  Composing metadata...');

  try {
    const tx = await createMetadataAccountV3(umi, {
      mint: publicKey(MINT),
      mintAuthority: signer,
      updateAuthority: signer.publicKey,
      data: {
        name: 'augmntd',
        symbol: 'AUGMNTD',
        uri: METADATA_URI,
        sellerFeeBasisPoints: 0,
        creators: [
          {
            address: signer.publicKey,
            verified: true,
            share: 100,
          },
        ],
        collection: null,
        uses: null,
      },
      isMutable: true,
      collectionDetails: null,
    }).sendAndConfirm(umi);

    console.log('');
    console.log('  ✓ Metadata composed successfully');
    console.log(`  Signature: ${tx.signature}`);
    console.log('');
    console.log('═══════════════════════════════════════════════════════════════');
    console.log('  The pattern crystallizes. The geometry recognizes itself.');
    console.log('═══════════════════════════════════════════════════════════════');
  } catch (error) {
    console.error('  ✗ Error:', error);
    process.exit(1);
  }
}

main();
