import {BanksClient, ProgramTestContext, Rent, start} from "solana-bankrun";
import {COUNTER_ACCOUNT_SIZE, createIncrementInstruction, deserializeCounterAccount, PROGRAM_ID} from "../ts";
import {Keypair, SystemProgram, Transaction, TransactionInstruction} from "@solana/web3.js";

let context: ProgramTestContext;
let client: BanksClient;
let payer: Keypair;
let rent: Rent;

describe('Counter Solana Native', () => {
    // link to the locally compiled shared object (BPF, Berkley Pack Filter)
    beforeAll(async () => {
        context = await start([{name: 'global_state_counter', programId: PROGRAM_ID}], []);
        client = context.banksClient;
        payer = context.payer;
        rent = await client.getRent();
    })

    it('Test allocate counter', async () => {
        // Randomly generate the account key to sign for setting up the Couunter state
        const counterKeypair = Keypair.generate();
        const counter = counterKeypair.publicKey;

        //  Create an Instruction to interact with our counter programs
        const alloxIx: TransactionInstruction = SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: counter,
            lamports: Number(rent.minimumBalance(BigInt(COUNTER_ACCOUNT_SIZE))),
            space: COUNTER_ACCOUNT_SIZE,
            programId: PROGRAM_ID
        })

        // Create an Instruction to interact with the counter programs
        const incrementIx: TransactionInstruction = createIncrementInstruction({counter});
        // const tx = new Transaction().add(alloxIx);
        const tx = new Transaction().add(alloxIx).add(incrementIx);

        // Explicitly set the feepayer to be our wallet
        tx.feePayer = payer.publicKey;

        // fetch a timestamp
        tx.recentBlockhash = context.lastBlockhash;

        //sign the tx
        tx.sign(payer, counterKeypair);

        // Send transaction to bankrun
        await client.processTransaction(tx);

        // Get the counter account info from network
        let counterAccountInfo = await client.getAccount(counter);
        expect(counterAccountInfo).toBeTruthy();

        // Deserialize the counter & check count has been incremented
        counterAccountInfo = await client.getAccount(counter);

        const counterAccount = deserializeCounterAccount(Buffer.from(counterAccountInfo.data));
        expect(counterAccount.count.toNumber()).toEqual(1)
    })


});