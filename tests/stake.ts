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




describe("STAKE", () => {

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
  console.log(("===================="));
  const collection = new anchor.web3.PublicKey("G5WvFzffVU2vLW7Eitym5ebobmFAkXfvtqgkdi2ZJprB");
  console.log(" COLLECTION ---->", collection);
  console.log(("===================="));
  const Mint = new anchor.web3.PublicKey("XTmPeWcMW7A88We4ShhmNcDiGVYhPrus1cfCMPmbrao");
  console.log(" Mint ---->", Mint);
  console.log(("===================="));
  const METADATA = new anchor.web3.PublicKey("C5dYDC5oYYRr3sHprDRd9o9uc9Rm8A8zuZtRK8RZtXDE");
  console.log(" METADATA ---->", METADATA);
  console.log(("===================="));
  const TokenAddress = new anchor.web3.PublicKey("4WqBecvoYktD34Pnowm3fWiGZBHE3nQ5j1MvqGmFskkN");
  console.log(" TOKEN ADDRESS ---->", TokenAddress);
  console.log(("===================="));
  const program = anchor.workspace.Stake as Program<Stake>;
  console.log("PROGRAM", program.programId.toBase58());




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




  it('IT STAKE', async () => {

    console.log("THE BEGINING OF IT STAKE ...");


    const [delegatedAuthPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("authority")
      ],
      program.programId
    );
    console.log(("===================="));
    console.log("DELAGATED AUTH PDA", delegatedAuthPda);


    const [UserStakeInfoPda ] = await anchor.web3.PublicKey.findProgramAddressSync(
      [wallet.publicKey.toBuffer(),
      TokenAddress.toBuffer()],
      program.programId
    );
    console.log(("===================="));
    console.log("USER STAKE INFO PDA", UserStakeInfoPda);

    //FIND PDA FOR METADATA
    const metadataAddress = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        Mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )[0];
    console.log("====================");
    console.log(`metadata initialized and its address ===> ${metadataAddress}`);


    //FIND PDA FOR MASTER EDITION
    const masterEditionAddress = (anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        Mint.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    ))[0];
    console.log("====================");
    console.log(
      `Master edition initialized and its address ===> ${masterEditionAddress}`);


    // const Program_Authority: anchor.web3.PublicKey = delegatedAuthPda;
    // let UserStakePda: anchor.web3.PublicKey = UserStakeInfoPda;
    // let nft: any
    // // let mintAuth: anchor.web3.PublicKey = ,
    // let mint: anchor.web3.PublicKey = Mint;
    // let NftAtokenAddress: anchor.web3.PublicKey = TokenAddress;
    // let nftMasteredition: anchor.web3.PublicKey = masterEditionAddress;


    try {
      let StakeIx = await program.methods
        .stakeSwap()
        .accounts({
          metadataProgram: TOKEN_METADATA_PROGRAM_ID,
          nftAEdition: masterEditionAddress,
          nftAMint: Mint,
          nftATokenAccount: TokenAddress,
          programAuthority: delegatedAuthPda,
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
      console.log("====================");
      console.log("INSTRUCTIONS ADDED TO STAKE TX");

      const blockhashData = await BLOCKHASH();
      const { blockhash, lastValidBlockHeight } = blockhashData;
      console.log("====================");
      console.log("RECENT BLOCKHASH =====>", blockhash);
      console.log("====================");
      console.log("lastValidBlockHeight =====>", lastValidBlockHeight);

      StakeTx.recentBlockhash = blockhash;
      StakeTx.feePayer = wallet.publicKey;


      try {

        const signature = await sendAndConfirmTransaction(provider.connection, StakeTx, [wallet.payer]);
        console.log("-----------------------");
        console.log("SEND AND CONFIRM STAKE TRANSACTION SIGNATURE =====>", signature);



        const confirmMintTx = await program.provider.connection.confirmTransaction({
          blockhash,
          lastValidBlockHeight,
          signature,
        });
        console.log("-----------------------");
        console.log("CONFIRM TRANSACTION =====>", confirmMintTx);




        const result = await provider.connection.getParsedTransaction(signature, "confirmed");
        console.log("-----------------------");
        console.log("STAKE TX RESULT =====>", result);
      } catch (Error) {
        console.log("ERROR IN STAKE TRY TX");
        console.error(Error);
      }

    } catch (Error) {
      console.log(`STAKE ERROR IN BIG PICTURE OF STAKE ${Error}`);
      console.error(Error)
    }
  });
});
