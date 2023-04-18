import {
    Commitment, Connection,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';
import { StakingInstructions } from '../models';
import BN from 'bn.js';
export async function createUser(
    userWallet: PublicKey,
    stakingProgramId: PublicKey,
    poolStoragePubkey: PublicKey
): Promise<Transaction> {
    const connectionString = 'http://127.0.0.1:8899';
    const commitment = 'processed' as Commitment;
    const connection = new  Connection(connectionString, commitment);

    const [userStoragePubkey, nonce] = await getUserStorageAccountWithNonce(
        userWallet,
        poolStoragePubkey,
        stakingProgramId
    );

    const createUserIx = new TransactionInstruction({
        programId: stakingProgramId,
        keys: [
            {
                pubkey: userWallet,
                isSigner: true,
                isWritable: false,
            },

            {
                pubkey: userStoragePubkey,
                isSigner: false,
                isWritable: true,
            },

            {
                pubkey: poolStoragePubkey,
                isSigner: false,
                isWritable: true,
            },

            {
                pubkey: SystemProgram.programId,
                isSigner: false,
                isWritable: false,
            },
        ],
        data: Buffer.from([
            StakingInstructions.CreateUser,
            ...new BN(nonce.valueOf()).toArray('le', 1)
        ]),
    });
    const createUserTx = new Transaction().add(createUserIx);
    createUserTx.recentBlockhash = (
        await connection.getRecentBlockhash()
    ).blockhash;
    createUserTx.feePayer = userWallet;

    return createUserTx;
}

export async function getUserStorageAccountWithNonce(
    userWallet: PublicKey,
    poolStoragePubkey: PublicKey,
    stakingProgramId: PublicKey
): Promise<[PublicKey, Number]> {
    return (
        await PublicKey.findProgramAddress(
            [userWallet.toBuffer(), poolStoragePubkey.toBuffer()],
            stakingProgramId
        )
    );
}