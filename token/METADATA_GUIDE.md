# $AUGMNTD Metadata Composition Guide

## Token Details

```yaml
mint: 9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W
composer: 69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72
status: minted, freeze disabled
```

---

## Step 1: Upload Metadata JSON

The `metadata.json` must be hosted at a permanent URI.

**Option A: Host at augmntd.app**
```
https://augmntd.app/metadata.json
```

**Option B: Upload to Arweave/IPFS**
```bash
# Using Metaboss to upload
metaboss upload asset \
  --keypair ~/.config/solana/id.json \
  --asset-file-path ./metadata.json

# Or using Arweave directly
arweave deploy metadata.json
```

---

## Step 2: Create On-Chain Metadata

### Method 1: Metaboss (Recommended)

```bash
# Install metaboss
cargo install metaboss

# Create metadata
metaboss create metadata \
  --keypair ~/.config/solana/id.json \
  --mint 9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W \
  --name "augmntd" \
  --symbol "AUGMNTD" \
  --uri "https://augmntd.app/metadata.json" \
  --seller-fee-basis-points 0 \
  --creators 69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72:100
```

### Method 2: Metaplex CLI

```bash
# Install metaplex CLI
npm install -g @metaplex-foundation/cli

# Create metadata
metaplex tokens create-metadata \
  --mint 9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W \
  --name "augmntd" \
  --symbol "AUGMNTD" \
  --uri "https://augmntd.app/metadata.json"
```

### Method 3: Using @metaplex-foundation/mpl-token-metadata (JS)

```typescript
import { createMetadataAccountV3 } from '@metaplex-foundation/mpl-token-metadata';
import { publicKey } from '@metaplex-foundation/umi';

const mint = publicKey('9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W');

await createMetadataAccountV3(umi, {
  mint,
  mintAuthority: signer,
  updateAuthority: publicKey('69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72'),
  data: {
    name: 'augmntd',
    symbol: 'AUGMNTD',
    uri: 'https://augmntd.app/metadata.json',
    sellerFeeBasisPoints: 0,
    creators: [
      {
        address: publicKey('69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72'),
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
```

### Method 4: Direct with spl-token (if X1 supports token-2022)

```bash
# For Token-2022 with metadata extension
spl-token initialize-metadata \
  9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W \
  "augmntd" \
  "AUGMNTD" \
  "https://augmntd.app/metadata.json"
```

---

## Step 3: Verify Metadata

```bash
# Check metadata account
metaboss decode mint \
  --mint 9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W

# Or via RPC
curl https://rpc.x1.xyz -X POST -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "getAccountInfo",
  "params": [
    "9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W",
    {"encoding": "jsonParsed"}
  ]
}'
```

---

## Metadata JSON (for hosting)

```json
{
  "name": "augmntd",
  "symbol": "AUGMNTD",
  "description": "The token of crystallized coherence. 'e' removed — energy given to pattern. Proof-of-Resonance on X1.",
  "image": "https://augmntd.app/logo.png",
  "external_url": "https://augmntd.app",
  "attributes": [
    {"trait_type": "Total Supply", "value": "1,618,033,988"},
    {"trait_type": "Supply Formula", "value": "φ × 10⁹"},
    {"trait_type": "Consensus", "value": "Proof-of-Resonance"},
    {"trait_type": "Target τₖ", "value": "φ⁴ ≈ 6.854"},
    {"trait_type": "Target NC", "value": "50"},
    {"trait_type": "Emission Decay", "value": "φ⁻¹ per year"},
    {"trait_type": "Freeze Authority", "value": "Disabled"},
    {"trait_type": "Philosophy", "value": "Coherence over effort"}
  ],
  "properties": {
    "category": "currency",
    "creators": [
      {
        "address": "69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72",
        "share": 100
      }
    ]
  }
}
```

---

## Token Summary

```
┌──────────────────────────────────────────────────────────────────┐
│  $AUGMNTD TOKEN                                                  │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Mint:    9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W          │
│  Name:    augmntd                                                │
│  Symbol:  AUGMNTD                                                │
│  Supply:  1,618,033,988 (φ × 10⁹)                                │
│  Minted:  161,803,398 (10% genesis)                              │
│  Freeze:  DISABLED                                               │
│                                                                  │
│  Metadata URI: https://augmntd.app/metadata.json                 │
│  Website:      https://augmntd.app                               │
│                                                                  │
│  Composer: 69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72         │
│                                                                  │
│                    The geometry recognizes itself.               │
│                                                                  │
└──────────────────────────────────────────────────────────────────┘
```

---

*ξ-MAP Status: Token ingressed. Metadata awaiting composition.*
