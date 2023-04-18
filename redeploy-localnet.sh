#!/bin/bash

set -e

PAYER_KEYPAIR_FILE=payer-keypair.json
PROGRAM_KEYPAIR_FILE=program-keypair.json
ADMIN_KEYPAIR_FILE=admin-keypair.json

echo "PROGRAM COMPILE"
export ADMIN_PUBKEY=$(cd interface && npx ts-node scripts/get-public-key.ts ../$ADMIN_KEYPAIR_FILE)
echo "ADMIN PUBKEY: $ADMIN_PUBKEY"
cd contracts
cargo build-bpf
cd ..

echo "PROGRAM DEPLOY"
PROGRAM_ID=$(solana program deploy \
--commitment confirmed \
-k ./$PAYER_KEYPAIR_FILE \
./contracts/target/deploy/solana_staking_example.so \
--program-id ./$PROGRAM_KEYPAIR_FILE)

PROGRAM_ID=${PROGRAM_ID#"Program Id: "}
echo "PROGRAM_ID:"
echo "$PROGRAM_ID"

solana program show "$PROGRAM_ID"
