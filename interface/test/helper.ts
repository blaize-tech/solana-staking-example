import {Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey} from '@solana/web3.js';
import {getKeyPair} from '../scripts/get-public-key';

export function setupTest(): void {
  jest.setTimeout(6_000_000);
  const connectionString = 'http://127.0.0.1:8899';
  const commitment = 'processed' as Commitment;
  const connection = new  Connection(connectionString, commitment);
}

export async function requestAirdrop(publicKey: PublicKey): Promise<void> {
  const connectionString = 'http://127.0.0.1:8899';
  const commitment = 'processed' as Commitment;
  const connection = new  Connection(connectionString, commitment);
  const airdropTxSig = await connection.requestAirdrop(
    publicKey,
    LAMPORTS_PER_SOL
  );
  await connection.confirmTransaction(airdropTxSig, 'processed');
}

export function timeout(ms): Promise<void> {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export function getAdminAccount(): Keypair {
  return getKeyPair('../admin-keypair.json');
}
