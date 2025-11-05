import { Buffer } from "node:buffer";
import {
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { start} from "solana-bankrun";
import { type BanksTransactionMeta} from "solana-bankrun";

class Assignable {
    constructor(properties) {
        for (const [key, value] of Object.entries(properties)) {
            this[key] = value;
        }
    }
}

class AddressInfo {
    name: string;
    house_number: number;
    street: string;
    city: string;

    constructor(fields: { name: string; house_number: number; street: string; city: string }) {
        this.name = fields.name;
        this.house_number = fields.house_number;
        this.street = fields.street;
        this.city = fields.city;
    }
}

const addressInfoSchema = {
    struct: {
        name: 'string',
        house_number: 'u8',
        street: 'string',
        city: 'string'
    }
};

let addressInfoAccount;
let PROGRAM_ID;
let context;
let client;


const fields = {
    name: "Joe C",
    house_number: 136,
    street: "Mile High Dr.",
    city: "Solana Beach",
};

describe("Account Data!",  () => {
    beforeAll( async ()=>{
         addressInfoAccount = Keypair.generate();
         PROGRAM_ID = PublicKey.unique();

        context = await start(
            [{ name: "account_data", programId: PROGRAM_ID }],
            [],
        );

        client = context.banksClient;

    })

    it("Create the address info account", async () => {


        const payer = context.payer;

        console.log(`Program Address      : ${PROGRAM_ID}`);
        console.log(`Payer Address      : ${payer.publicKey}`);
        console.log(`Address Info Acct  : ${addressInfoAccount.publicKey}`);


        const addressInfo = new AddressInfo(fields);

        const serializedData = Buffer.from(borsh.serialize(addressInfoSchema, addressInfo));

        const ix = new TransactionInstruction({
            keys: [
                {
                    pubkey: addressInfoAccount.publicKey,
                    isSigner: true,
                    isWritable: true,
                },
                { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            programId: PROGRAM_ID,
            data: serializedData,
        });

        const blockhash = context.lastBlockhash;

        const tx = new Transaction();
        tx.recentBlockhash = blockhash;
        tx.add(ix).sign(payer, addressInfoAccount);
        const tx_program: BanksTransactionMeta = await client.processTransaction(tx);

        console.log('tx', tx_program.logMessages);

        expect(tx_program.logMessages[4]).toBe("Program 11111111111111111111111111111112 success")

    });

    it("Read the new account's data", async () => {

        const accountInfo = await client.getAccount(addressInfoAccount.publicKey);

        const readAddressInfo = borsh.deserialize(
            addressInfoSchema,
            Buffer.from(accountInfo.data),
        ) as AddressInfo;

        expect(readAddressInfo.name).toBe(fields.name)
        expect(readAddressInfo.house_number).toBe(fields.house_number)
        expect(readAddressInfo.street).toBe(fields.street)
        expect(readAddressInfo.city).toBe(fields.city)
    });
});
