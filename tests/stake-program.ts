import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakeProgram } from "…/target/types/stake_program";
import { Connection, PublicKey, Keypair } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token";
import { min } from "bn.js";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const payer = provider.wallet as anchor.Wallet;
const mintKeypair = Keypair.fromSecretKey(new Uint8Array([241, 54, 124, 229, 181, 1, 153, 130, 247, 143, 116, 195, 112, 73, 180, 65, 142, 1, 23, 59, 156, 160, 35, 153, 165, 37, 184, 146, 215, 162, 30, 195, 10, 50, 11, 235, 118, 208, 67, 186, 74, 76, 173, 247, 37, 67, 141, 132, 170, 17, 84, 105, 60, 0, 56, 122, 23, 196, 59, 85, 160, 13, 25, 178]));

const connection = new Connection("http://127.0.0.1:8899", "confirmed")
const program = anchor.workspace.StakeProgram as Program<StakeProgram>;

async function createMintToken() {
await createMint(
connection,
payer.payer,
payer.publicKey,
payer.publicKey,
8,
mintKeypair
);
}

describe("stake-program", () => {
// Configure the client to use the local cluster.

it("Is initialized!", async () => {
// Add your test here.

await createMintToken();

let [stakePool] = PublicKey.findProgramAddressSync(
  [Buffer.from("stake_pool")],
  program.programId
)

const tx = await program.methods.initialize()
  .accounts({
    signer: payer.publicKey,
    stakePoolAccount: stakePool,
    mint: mintKeypair.publicKey,
}).rpc({skipPreflight: true});
console.log("Your transaction signature", tx);
});

it("stake", async() => {
let userTokenAccount = await getOrCreateAssociatedTokenAccount(
connection,
payer.payer,
mintKeypair.publicKey,
payer.publicKey
)

await mintTo(
  connection,
  payer.payer,
  mintKeypair.publicKey,
  userTokenAccount.address,
  payer.payer,
  1e11
)


let [stakeInfo] = PublicKey.findProgramAddressSync(
  [Buffer.from("stake_info"), payer.publicKey.toBuffer()],
  program.programId
)

let [playerStakeTokenAccount] = PublicKey.findProgramAddressSync(
  [Buffer.from("token"), payer.publicKey.toBuffer()],
  program.programId
)

await getOrCreateAssociatedTokenAccount(
  connection,
  payer.payer,
  mintKeypair.publicKey,
  payer.publicKey
)

console.log(mintKeypair.publicKey.toBase58());
console.log(payer.publicKey.toBase58())
console.log(playerStakeTokenAccount.toBase58())
console.log(userTokenAccount.address.toBase58())

const tx = await program.methods
  .stake(new anchor.BN(1))
  .signers([payer.payer])
  .accounts({
    stakeInfo: stakeInfo,
    signer: payer.publicKey,
    mint: mintKeypair.publicKey,
    playerStakeTokenAccount: playerStakeTokenAccount,
    playerTokenAccount: userTokenAccount.address
  })
  .rpc( {skipPreflight: true});
})

it("destake", async () => {
let userTokenAccount = await getOrCreateAssociatedTokenAccount(
connection,
payer.payer,
mintKeypair.publicKey,
payer.publicKey
)

let [stakeInfo] = PublicKey.findProgramAddressSync(
  [Buffer.from("stake_info"), payer.publicKey.toBuffer()],
  program.programId
)

let [playerStakeTokenAccount] = PublicKey.findProgramAddressSync(
  [Buffer.from("token"), payer.publicKey.toBuffer()],
  program.programId
)

await getOrCreateAssociatedTokenAccount(
  connection,
  payer.payer,
  mintKeypair.publicKey,
  payer.publicKey
)

let [stakePool] = PublicKey.findProgramAddressSync(
  [Buffer.from("stake_pool")],
  program.programId
)

await mintTo(
  connection,
  payer.payer,
  mintKeypair.publicKey,
  stakePool,
  payer.payer,
  1e11
)
const tx = await program.methods
.destake()
.signers([payer.payer])
.accounts({
stakeInfo: stakeInfo,
signer: payer.publicKey,
mint: mintKeypair.publicKey,
stakePoolAccount: stakePool,
playerStakeTokenAccount: playerStakeTokenAccount,
playerTokenAccount: userTokenAccount.address
})
.rpc( {skipPreflight: true});
})
});