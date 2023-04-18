import {getAdminAccount, requestAirdrop} from "./helper";
const {getPublicKey} = require("../scripts/get-public-key");
import {Commitment, Connection, Keypair, PublicKey, sendAndConfirmTransaction, Transaction} from "@solana/web3.js";
import {ASSOCIATED_TOKEN_PROGRAM_ID, Token, TOKEN_PROGRAM_ID} from '@solana/spl-token';

const adminAccount: Keypair = getAdminAccount();
const walletAccount: Keypair = Keypair.generate();
let poolStorageAccount: Keypair;
let stakingProgramId: PublicKey = getPublicKey('../program-keypair.json');
let splAssociatedTokenAccountProgramId: PublicKey;


async function setupEnvironment() {
    const connectionString = 'http://127.0.0.1:8899';
    const commitment = 'processed' as Commitment;
    const connection = new Connection(connectionString, commitment);
    poolStorageAccount = Keypair.generate();

    splAssociatedTokenAccountProgramId = new PublicKey(
        'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL'
    );
    await requestAirdrop(adminAccount.publicKey);
    await requestAirdrop(walletAccount.publicKey);


    const userTokenData = await findAssociatedTokenAddress(
        walletAccount.publicKey,
        stakingProgramId,
        splAssociatedTokenAccountProgramId
    );

    /*const userDataInfo = await connection.getAccountInfo(userTokenData);
    const doesUserDataExist = userDataInfo?.owner !== undefined;
    if (!doesUserDataExist) {
        const createUserDataIx = Token.createAssociatedTokenAccountInstruction(
            ASSOCIATED_TOKEN_PROGRAM_ID,
            TOKEN_PROGRAM_ID,
            stakingProgramId,
            userTokenData,
            walletAccount.publicKey,
            walletAccount.publicKey
        );
        const createUserDataTx = new Transaction().add(createUserDataIx);
        await sendAndConfirmTransaction(connection, createUserDataTx, [
            walletAccount,
        ]);
    }*/
}

export {
    adminAccount,
    walletAccount,
    setupEnvironment,
    poolStorageAccount,
    stakingProgramId
}

export async function findAssociatedTokenAddress(
    walletAddress: PublicKey,
    tokenMintAddress: PublicKey,
    splAssociatedTokenAccountProgramId: PublicKey
): Promise<PublicKey> {
    return (
        await PublicKey.findProgramAddress(
            [
                walletAddress.toBuffer(),
                TOKEN_PROGRAM_ID.toBuffer(),
                tokenMintAddress.toBuffer(),
            ],
            splAssociatedTokenAccountProgramId
        )
    )[0];
}