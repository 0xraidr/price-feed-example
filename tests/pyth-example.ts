import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PythExample } from "../target/types/pyth_example";
import { secretKey } from "./keypair";
import { PublicKey,  Keypair} from "@solana/web3.js";

describe("pyth-example", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.PythExample as Program<PythExample>;

  // Declaring the user/signer
  const userSecretKey = new Uint8Array(secretKey);
  const userKeypair = Keypair.fromSecretKey(userSecretKey);

  // Declaring the SOL/USD price feed address for Devnet
  const solUsd =  new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix")


  it("Fetch Price!", async () => {

    const tx = await program.methods.fetchPrice().accounts({
      signer: userKeypair.publicKey,
      priceFeed: solUsd,
    }).signers([userKeypair])
    .rpc()
    .then(confirmTx);
  });

  const confirmTx = async (signature: string) => {
    const latestBlockhash = await anchor
      .getProvider()
      .connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction(
      {
        signature,
        ...latestBlockhash,
      },
      "confirmed"
    );
    console.log("Tx Signature:", signature);
    return signature;
  };

});
