import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CryptoMail } from "../target/types/crypto_mail";
import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";

describe("Create Data Account", () => {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CryptoMail as Program<CryptoMail>;
  const programId = program.programId;

  const sender = PublicKey.findProgramAddressSync(
    [payer.publicKey.toBuffer()],
    programId
  )[0];

  const receiver = PublicKey.findProgramAddressSync(
    [payer.publicKey.toBuffer()],
    programId
  )[0];

  const mail = PublicKey.findProgramAddressSync(
    [payer.publicKey.toBuffer(), new anchor.BN(0).toBuffer('le', 2)],
    programId
  )[0];

  it("Mail send!", async () => {
    const tx = await program.methods
      .sendMail("This is en email in the Solana blockchain")
      .accounts({
        mail: mail,
        sender: sender,
        receiver: receiver,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
})