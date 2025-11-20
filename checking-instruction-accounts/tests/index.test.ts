import {LiteSVM} from 'litesvm';
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction,} from "@solana/web3.js";

describe('check account instruction', () => {
    it('checks for the valid programId', async () => {
        const svm = new LiteSVM();
        const payer = Keypair.generate();
        const accountToChange = Keypair.generate()
        const accountToCreate = Keypair.generate()
        const rent = svm.getRent()
        svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));

        const programId = PublicKey.unique();
        svm.addProgramFromFile(programId, "./program/target/deploy/checking_instruction_accounts.so")

        const ix = SystemProgram.createAccount({
            programId,
            lamports: 0,
            newAccountPubkey: accountToChange.publicKey,
            space: 0,
            fromPubkey: payer.publicKey
        });

        const tx = new Transaction().add(ix);
        tx.recentBlockhash = svm.latestBlockhash();
        tx.sign(payer, accountToChange)

        const txRes = svm.sendTransaction(tx)


    })
})