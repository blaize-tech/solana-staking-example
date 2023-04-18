import {
    Commitment,
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';
import BN from 'bn.js';
import {StakingInstructions} from "../models";

export async function createInitializePool(
    poolOwnerWallet: PublicKey,
    poolStorageAccount: Keypair,
    stakingProgramId: PublicKey,
    nonce: number,
): Promise<Transaction> {
    const connectionString = 'http://127.0.0.1:8899';
    const commitment = 'processed' as Commitment;
    const connection = new  Connection(connectionString, commitment);

    const poolStorageBytes = 94;
    const rentPrice = await connection.getMinimumBalanceForRentExemption(
        poolStorageBytes,
        'confirmed'
    );
    const createPoolStorageAccountIx = SystemProgram.createAccount({
        space: poolStorageBytes,
        lamports: rentPrice,
        fromPubkey: poolOwnerWallet,
        newAccountPubkey: poolStorageAccount.publicKey,
        programId: stakingProgramId,
    });
    const balance = await connection.getBalance(poolOwnerWallet);
    if (balance < rentPrice)
        throw new Error(
            `Need at least ${rentPrice / LAMPORTS_PER_SOL
            } SOL for contest account rent`
        );

    const initPoolStorageAccountIx = new TransactionInstruction({
        programId: stakingProgramId,
        keys: [
            {
                pubkey: poolOwnerWallet,
                isSigner: true,
                isWritable: false,
            },
            {
                pubkey: poolStorageAccount.publicKey,
                isSigner: false,
                isWritable: true,
            }
        ],
        data: Buffer.from([
            StakingInstructions.InitializePool,
            ...new BN(nonce.valueOf()).toArray('le', 1),
        ])
    });

    const transaction = new Transaction().add(
        createPoolStorageAccountIx,
        initPoolStorageAccountIx
    );

    transaction.recentBlockhash = (
        await connection.getRecentBlockhash()
    ).blockhash;
    transaction.feePayer = poolOwnerWallet;

    return transaction;
}