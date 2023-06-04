import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stake } from "../target/types/stake";
import {
  ComputeBudgetProgram,
  Connection, Keypair, LAMPORTS_PER_SOL,
  PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram,
  Transaction, TransactionInstruction,
  sendAndConfirmRawTransaction,
  sendAndConfirmTransaction
} from "@solana/web3.js"

import {
  bundlrStorage,
  keypairIdentity,
  Metaplex,
} from "@metaplex-foundation/js"
import { TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddress } from "@solana/spl-token"




describe("stake", () => {

  console.log(`
  ==============================
  THE BEGINING OF DESCRIBE `);

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const ASSOCIATED_TOKEN_PROGRAM = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(anchor.AnchorProvider.env());
  const walletPubKey = anchor.AnchorProvider.local().wallet.publicKey;
  console.log(("===================="));
  console.log(" WALLET PUBKEY ---->", walletPubKey);

  const program = anchor.workspace.Stake as Program<Stake>;


 


  

  const BLOCKHASH = async () => {
    const { blockhash, lastValidBlockHeight } = await program.provider.connection.getLatestBlockhash("finalized");
    return {
      blockhash: blockhash,
      lastValidBlockHeight: lastValidBlockHeight
    }
  };

  const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
    units: 1000000
  });

  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
    microLamports: 1
  });


  let Program_Authority: anchor.web3.PublicKey
  let UserStakeInfoPda: anchor.web3.PublicKey
  let nft: any
  let mintAuth: anchor.web3.PublicKey
  let mint: anchor.web3.PublicKey
  let NftAtokenAddress: anchor.web3.PublicKey
  let nftMasteredition: anchor.web3.PublicKey


  it('IT STAKE', async () => {

    try {
      let StakeIx = await program.methods
        .stakeSwap()
        .accounts({
          metadataProgram: TOKEN_METADATA_PROGRAM_ID,
          nftAEdition: nftMasteredition,
          nftAMint: mint,
          nftATokenAccount: NftAtokenAddress,
          programAuthority: Program_Authority,
          stakeVault: UserStakeInfoPda,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          user: wallet.publicKey,
        })
        .signers([wallet.payer])
        .instruction()

      const StakeTx = new Transaction()
        .add(addPriorityFee)
        .add(modifyComputeUnits)
        .add(StakeIx)
      console.log("INSTRUCTIONS ADDED TO STAKE TX");

      const blockhashData = await BLOCKHASH();
      const { blockhash, lastValidBlockHeight } = blockhashData;
      console.log("-----------------------");
      console.log("RECENT BLOCKHASH =====>", blockhash);
      console.log("-----------------------");
      console.log("lastValidBlockHeight =====>", lastValidBlockHeight);

      StakeTx.recentBlockhash = blockhash;
      StakeTx.feePayer = wallet.publicKey;


      try {
        const sendStakeTx = await sendAndConfirmTransaction(provider.connection, StakeTx, [wallet.payer]);
        console.log("-----------------------");
        console.log("SEND AND CONFIRM STAKE TRANSACTION SIGNATURE =====>", sendStakeTx);

        const result = await provider.connection.getParsedTransaction(sendStakeTx, "confirmed");
        console.log("-----------------------");
        console.log("STAKE TX RESULT =====>", result);

      } catch (error) {
        console.log("ERROR IN STAKE TRY TX");
        console.error(Error);
      }

    } catch (error) {
      console.log(`STAKE ERROR IN BIG PICTURE MINT ${error}`);
    }


  });
});
