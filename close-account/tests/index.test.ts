import {Keypair, PublicKey} from "@solana/web3.js";
import {LiteSVM} from "litesvm";

describe('close-account', () => {
    const payer = Keypair.generate();
    const programId = PublicKey.unique();
    const svm = new LiteSVM()
    svm.addProgramFromFile(programId, "tests/fixtures/close_account.so");


    it('closes an account', () => {


    });


})