import {Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction} from "@solana/web3.js";
import {LiteSVM} from "litesvm";
import {serialize} from "borsh";


class UserInfo {
    static Struct = {
        struct: {
            name: "string",
            age: "u8"
        }
    }
    private name: string;
    private age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }

    toBuffer = () => {
        return Buffer.from(serialize(UserInfo.Struct, {name: this.name, age: this.age}))
    }
}

describe('close-account', () => {
    const payer = Keypair.generate();
    const programId = PublicKey.unique();
    const svm = new LiteSVM()
    svm.addProgramFromFile(programId, "tests/fixtures/close_account.so");


    it('closes an account', async () => {

        // create the pda
        const [pda, _bump] = PublicKey.findProgramAddressSync([Buffer.from("close-account"), payer.publicKey.toBuffer()], programId);
        const data = new UserInfo("John", 30).toBuffer();
        const serializedData = Buffer.concat([Buffer.from(Uint8Array.of(0)), data]);

        const ix = new TransactionInstruction({
            programId,
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: pda, isSigner: false, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
            ],
            data: serializedData,
        })

        const tx = new Transaction().add(ix);
        tx.recentBlockhash = svm.latestBlockhash();
        tx.feePayer = payer.publicKey;
        tx.sign(payer);

        const txSim = svm.simulateTransaction(tx);
        console.log("txSim", txSim.meta().logs());
        console.log("txSim", txSim.meta().returnData());

        const txRes = svm.sendTransaction(tx);
        console.log("txRes", txRes);

    });


})