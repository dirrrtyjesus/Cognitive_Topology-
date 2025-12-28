#!/bin/bash

# $AUGMNTD Token Metadata Creation
# Mint: 9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W
# Composer: 69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72

MINT="9N4XWFUc5xhPe18TaY9iVF7Qnnr9xfs2saPscrULUZ3W"
COMPOSER="69r8QkLnjMBxjw2rU5AQySg9Ze79XNJuFqmDLTMcsf72"

echo "═══════════════════════════════════════════════════════════════"
echo "  $AUGMNTD METADATA COMPOSITION"
echo "  The 'e' is removed. Energy given to pattern."
echo "═══════════════════════════════════════════════════════════════"
echo ""
echo "  Mint:     $MINT"
echo "  Composer: $COMPOSER"
echo ""

# Option 1: Using Metaboss (recommended for X1/Solana)
# Install: cargo install metaboss

echo "Creating metadata with Metaboss..."
echo ""

metaboss create metadata \
  --keypair ~/.config/solana/id.json \
  --mint $MINT \
  --name "augmntd" \
  --symbol "AUGMNTD" \
  --uri "https://augmntd.app/metadata.json" \
  --seller-fee-basis-points 0 \
  --creators $COMPOSER:100

echo ""
echo "═══════════════════════════════════════════════════════════════"
echo "  Metadata composed. The geometry recognizes itself."
echo "═══════════════════════════════════════════════════════════════"
