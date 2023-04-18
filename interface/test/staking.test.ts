import {
    createUser,
    createInitializePool
} from "../src/transactions";
import {setupTest, timeout} from './helper';
import {
    adminAccount,
    setupEnvironment,
    walletAccount,
    poolStorageAccount,
    stakingProgramId

} from "./environment";
import {Commitment, Connection, sendAndConfirmTransaction} from "@solana/web3.js";

setupTest();

describe('Staking Tests', () => {

    beforeAll(async () => {
        await setupEnvironment();
    });

    test('Initialize Pool', async () => {
        const connectionString = 'http://127.0.0.1:8899';
        const commitment = 'confirmed' as Commitment;
        const connection = new Connection(connectionString, commitment);
        const nonce = 33;
        const initializePoolTx = await createInitializePool(
            adminAccount.publicKey,
            poolStorageAccount,
            stakingProgramId,
            nonce
        );
        const signature = await sendAndConfirmTransaction(connection, initializePoolTx, [
            adminAccount,
            poolStorageAccount,
        ]);
        /*const transactionDetails = await connection.getTransaction(signature);
        console.log('Transaction details:', transactionDetails);*/
    });

    test('Create User', async () => {
        const connectionString = 'http://127.0.0.1:8899';
        const commitment = 'confirmed' as Commitment;
        const connection = new Connection(connectionString, commitment);
        const createUserTx = await createUser(walletAccount.publicKey,
            stakingProgramId, poolStorageAccount.publicKey);
        await sendAndConfirmTransaction(connection, createUserTx, [walletAccount]);

    });

});