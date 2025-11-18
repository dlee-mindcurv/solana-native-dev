import {Blockhash, Keypair, PublicKey} from "@solana/web3.js";
import {BanksClient, ProgramTestContext, start} from "solana-bankrun";

const FavInstruction = {
    CreateFav: 0,
    GetFav: 1
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
        payer = context.payer
        blockhash = context.lastBlockhash;
    });

    test("Sets the Favourite PD and cross-check the updated data", async () => {

        console.log('Buffer.from("favorite")', Buffer.from("favorite"))
        console.log('payer.publicKey.toBuffer()', payer.publicKey.toBuffer())


        const favoritesPda = PublicKey.findProgramAddressSync([
            Buffer.from("favorite"), payer.publicKey.toBuffer()
        ], programId)

        const favData = {
            instruction: FavInstruction.CreateFav,
            number: 42,
            color: "blue",
            hobbies: ["coding", "reading", "traveling"]
        }


    })


})