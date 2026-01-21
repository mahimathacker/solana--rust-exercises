// migrations/deploy.ts

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Oracle } from "../target/types/oracle";

module.exports = async function (provider: anchor.AnchorProvider) {
  // Configure client to use the provider
  anchor.setProvider(provider);

  // Get the program
  const program = anchor.workspace.Oracle as Program<Oracle>;

  console.log("=".repeat(50));
  console.log("ORACLE DEPLOYMENT");
  console.log("=".repeat(50));
  console.log("Network:", provider.connection.rpcEndpoint);
  console.log("Wallet:", provider.wallet.publicKey.toBase58());
  console.log("Program ID:", program.programId.toBase58());

  // Create an oracle account
  const oracleAccount = anchor.web3.Keypair.generate();
  const initialPrice = new anchor.BN(100);

  console.log("\nInitializing oracle...");

  await program.methods
    .init(initialPrice)
    .accounts({
      payer: provider.wallet.publicKey,
      owner: provider.wallet.publicKey,
      oracle: oracleAccount.publicKey,
  })
    .signers([oracleAccount])
    .rpc();

  console.log("Oracle account:", oracleAccount.publicKey.toBase58());
  console.log("Initial price:", initialPrice.toNumber());

  // Verify it worked
  const oracle = await program.account.oracle.fetch(oracleAccount.publicKey);
  console.log("\nVerification:");
  console.log("  Owner:", oracle.owner.toBase58());
  console.log("  Price:", oracle.price.toNumber());

  console.log("\n" + "=".repeat(50));
  console.log("DEPLOYMENT COMPLETE");
  console.log("=".repeat(50));
};