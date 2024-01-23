import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AccountTest } from "../target/types/account_test";
import { assert } from "chai";

describe("account-test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AccountTest as Program<AccountTest>;

  it("Is initialized!", async () => {
    const user = anchor.web3.Keypair.generate();
    const connection = program.provider.connection;
    await connection.confirmTransaction(
      await connection.requestAirdrop(user.publicKey, 1_000_000_000)
    );
    
    await program.methods.initialize().accounts({
      user: user.publicKey,
    }).signers([user]).rpc();
    
    await program.methods.update().accounts({
      user: user.publicKey,
    }).signers([user]).rpc();

    const accountKey = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("my_account")
    ], program.programId)[0];

    const myAccount = await program.account.myAccount.fetch(accountKey);
    console.log({ myAccount })
    assert.isTrue(myAccount.flag);
  });
});
