import {PublicKey, Transaction, TransactionInstruction} from '@solana/web3.js';
import {start} from 'solana-bankrun';

describe('account-data', () => {
    it('Runs the account-data program!', async () => {
        const PROGRAM_ID = PublicKey.unique();
        const context = await start([{name: 'account_data', programId: PROGRAM_ID}], []);
        const client = context.banksClient;


        const payer = context.payer;
        const blockhash = context.lastBlockhash;
        // We set up our instruction first.
        const ix = new TransactionInstruction({
            keys: [{pubkey: payer.publicKey, isSigner: true, isWritable: true}],
            programId: PROGRAM_ID,
            data: Buffer.alloc(0), // No data
        });

        const tx = new Transaction();
        tx.recentBlockhash = blockhash;
        tx.add(ix).sign(payer);

        // Now we process the transaction
        await client.processTransaction(tx);

        // expect(transaction.logMessages[0].startsWith(`Program ${PROGRAM_ID}`)).toBe(true);
        // expect(transaction.logMessages[1]).toBe('Program log: Hello, Solana!');
        // expect(transaction.logMessages[2]).toBe(`Program log: Our program's ID is: ${PROGRAM_ID}`);
        // expect(transaction.logMessages[3].startsWith(`Program ${PROGRAM_ID} consumed`)).toBe(true);
        // expect(transaction.logMessages[4]).toBe(`Program ${PROGRAM_ID} success`);
        // expect(transaction.logMessages).toHaveLength(5);

    });
});