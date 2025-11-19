import {Blockhash, Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction} from "@solana/web3.js";
import {BanksClient, ProgramTestContext, start} from "solana-bankrun";
import {deserialize, serialize} from "borsh";

const FavInstruction = {
    CreateFav: 0,
    GetFav: 1
}

const FavStruct = {
    struct: {
        number: "u64",
        color: "string",
        hobbies: {
            array: {
                type: "string"
            }
        }
    }
}

type Fav = {
    number: number,
    color: string,
    hobbies: Array<string>
}

describe("Testing user-pda-accounts", () => {

    const programId = PublicKey.unique();

    let context: ProgramTestContext;
    let client: BanksClient;
    let payer: Keypair;
    let blockhash: Blockhash

    beforeEach(async () => {
        context = await start([{name: "user_pda_accounts", programId}], []);
        client = context.banksClient;
        payer = context.payer;
        blockhash = context.lastBlockhash;
    });

    test("Set the favorite pda and cross-check the updated data", async () => {

        const favoritesPda = PublicKey.findProgramAddressSync([
            Buffer.from("favorite"), payer.publicKey.toBuffer()
        ], programId)[0]

        console.log(1, payer.publicKey)


        const favData = {
            number: 42,
            color: "blue",
            hobbies: ["coding", "reading", "traveling"]
        }

        const instructionData = Buffer.concat([
            Buffer.from(Uint8Array.of(FavInstruction.CreateFav)), // 0 = CreatePda
            Buffer.from(serialize(FavStruct, favData))
        ]);

        const ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: favoritesPda, isSigner: false, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId,
            data: instructionData
        })

        const tx1 = new Transaction().add(ix);
        tx1.feePayer = payer.publicKey;
        tx1.recentBlockhash = blockhash;
        tx1.sign(payer);
        tx1.recentBlockhash = blockhash;
        await client.processTransaction(tx1);

        const account = await client.getAccount(favoritesPda);
        const data = Buffer.from(account.data);
    })

    it("Check if the test fails if the pda seeds aren't same", async () => {
        // Although the programId is the same, the difference here is the payer.... because the pda address is derived
        // from programId and the payer's publicKey, the program should reject this
        const favoritePda = PublicKey.findProgramAddressSync([
            // SEED HERE IS DIFFERENT
            Buffer.from("favorites"), payer.publicKey.toBuffer()
        ], programId)[0];

        const favData = {
            number: 42,
            color: "blue",
            hobbies: ["coding", "reading", "traveling"]
        }
        const serializedData = Buffer.from(serialize(FavStruct, favData));
        const descriptor = Buffer.from(Uint8Array.of(FavInstruction.CreateFav))
        const instructionData = Buffer.concat([descriptor, serializedData]);

        const ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: favoritePda, isSigner: false, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false}
            ],
            programId,
            data: instructionData
        });

        const tx = new Transaction().add(ix);
        tx.feePayer = payer.publicKey;
        tx.recentBlockhash = blockhash;
        tx.sign(payer)
        tx.recentBlockhash = blockhash;

        await expect(client.processTransaction(tx)).rejects.toThrow();
    })

    it("Gets user account data that was previously added", async () => {
        const [favoritePda, _favoriteBump] = PublicKey.findProgramAddressSync(
            [Buffer.from("favorite"), payer.publicKey.toBuffer()], programId
        )

        const favData = {
            number: 43,
            color: "brown",
            hobbies: ["chess", "video games", "horse riding"]
        }

        const ixData = Buffer.concat([
            Buffer.from(Uint8Array.of(FavInstruction.CreateFav)),
            Buffer.from(serialize(FavStruct, favData))
        ])

        const ix = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: favoritePda, isSigner: false, isWritable: true},
                {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},

            ],
            programId,
            data: ixData
        })

        const tx = new Transaction().add(ix);
        tx.feePayer = payer.publicKey;
        tx.recentBlockhash = blockhash;
        tx.sign(payer)
        tx.recentBlockhash = blockhash;

        const tex = await client.processTransaction(tx);

        expect(tex).not.toBeFalsy();

        const ixDataGet = Buffer.from(Uint8Array.of(FavInstruction.GetFav));

        const ixGet = new TransactionInstruction({
            keys: [
                {pubkey: payer.publicKey, isSigner: true, isWritable: true},
                {pubkey: favoritePda, isSigner: false, isWritable: true},
            ],
            programId,
            data: ixDataGet
        });

        const txGet = new Transaction().add(ixGet);
        txGet.feePayer = payer.publicKey;
        txGet.recentBlockhash = blockhash;
        txGet.sign(payer);
        txGet.recentBlockhash = blockhash;

        const getRes = client.sendTransaction(txGet);

        expect(getRes).not.toBeFalsy();

        const account = await client.getAccount(favoritePda);

        const savedData = Buffer.from(account.data)

        const serializedData = deserialize(FavStruct, savedData) as Fav;

        expect(serializedData.number).toBe(BigInt(43));
        expect(serializedData.color).toBe("brown");
        expect(serializedData.hobbies.includes("horse riding")).toBe(true);


    })

})