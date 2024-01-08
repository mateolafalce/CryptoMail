import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CryptoMail } from "../target/types/crypto_mail";
import { SystemProgram, PublicKey } from "@solana/web3.js";

describe("Create Mail Account", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace.CryptoMail as Program<CryptoMail>;
  const programId = program.programId;
  const account = PublicKey.findProgramAddressSync(
    [payer.publicKey.toBuffer()],
    programId
  )[0];
  it("Is initialized!", async () => {
    const tx = await program.methods
      .initCryptoMail()
      .accounts({
        account: account,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});
