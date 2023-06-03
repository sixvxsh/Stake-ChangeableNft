import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stake } from "../target/types/stake";

describe("stake", () => {


  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(anchor.AnchorProvider.env());
  const walletPubKey = anchor.AnchorProvider.local().wallet.publicKey;
  console.log(("===================="));
  console.log(" WALLET PUBKEY ---->" , walletPubKey);


  const
  
  








});
