import {LiteSVM, TransactionMetadata} from 'litesvm';
import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";

describe('check account instruction', () => {
    const svm = new LiteSVM();
    const programId = PublicKey.unique();
    const payer = Keypair.generate();
    const accountToChange = Keypair.generate()
    const accountToCreate = Keypair.generate()
    svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));

    it('creates an account for the programs', async () => {
        svm.addProgramFromFile(programId, "./programs/target/deploy/checking_instruction_accounts.so")

        const initialAccountSpace = 0;
        const lamports = svm.minimumBalanceForRentExemption(BigInt(initialAccountSpace));

        const ix = SystemProgram.createAccount({
            programId,
            lamports: Number(lamports),
            newAccountPubkey: accountToChange.publicKey,
            space: initialAccountSpace,
            fromPubkey: payer.publicKey
        });

        const tx = new Transaction().add(ix);
        tx.recentBlockhash = svm.latestBlockhash();
        tx.sign(payer, accountToChange);

        const txRes = svm.sendTransaction(tx);
        expect(txRes instanceof TransactionMetadata).toBe(true);
        const accountFound = svm.getAccount(accountToChange.publicKey);
        expect(accountFound.lamports).toEqual(890880);
    });

    it('checks the accounts on the programs id', () => {

        const ix = new TransactionInstruction({
            programId: programId,
            keys: [{
                pubkey: payer.publicKey, isSigner: true, isWritable: true,
            }, {
                pubkey: accountToChange.publicKey, isSigner: false, isWritable: true

            }, {
                pubkey: SystemProgram.programId, isSigner: false, isWritable: false
            },
                {pubkey: accountToCreate.publicKey, isSigner: true, isWritable: true}
            ],
        })
        const tx = new Transaction().add(ix);
        tx.recentBlockhash = svm.latestBlockhash();
        tx.feePayer = payer.publicKey
        tx.sign(payer, accountToCreate);

        const simRes = svm.simulateTransaction(tx);
        console.log('simRes', JSON.stringify(simRes.meta().logs(), null, 2))


        const txResult = svm.sendTransaction(tx);
        expect(txResult instanceof TransactionMetadata).toBe(true);

        const addedCreatedAccount = svm.getAccount(accountToCreate.publicKey);
        console.log('addedCreatedAccount', addedCreatedAccount);
        expect(addedCreatedAccount.owner.toBase58()).toBe(programId.toBase58());

    })
})