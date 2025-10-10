import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Lesson3 } from "../target/types/lesson_3";

describe("lesson_3", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.lesson3 as Program<Lesson3>;

  const signer = anchor.web3.Keypair.generate(); // this will generate completely new Solana wallet with no data, no lamports, owned by Solana program
  const data_account = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    await program.provider.connection.confirmTransaction(
      await program.provider.connection.requestAirdrop(
        signer.publicKey,
        100*anchor.web3.LAMPORTS_PER_SOL
      ),
      "confirmed"
    )

    // Add your test here.
    const tx = await program.methods.initialize("Hello Solana!").accounts({ // accounts that enter program
      signer: signer.publicKey, // freshly generated account
      dataAccount: data_account.publicKey // freshly generated account
    })
    .signers([signer, data_account]) // authorazing initialization
    .rpc();

    console.log("Your transaction signature", tx);

    // declaring new variable and fetching account's data
    const dataAccount = await program.account.dataAccountWhatever.fetch(data_account.publicKey);

    console.log("Data Account: ", tx);
  });
});
