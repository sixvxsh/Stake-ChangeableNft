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
  console.log(" WALLET PUBKEY(USER 1) ---->", walletPubKey);

  console.log(("===================="));
  const user_b = new anchor.web3.PublicKey("GpFuzeBf6oQm98fiTr375rb8HfmgjQA2nU7CwZgG7dtC");
  console.log(" COLLECTION ---->", user_b);

  console.log(("===================="));
  const collection = new anchor.web3.PublicKey("G5WvFzffVU2vLW7Eitym5ebobmFAkXfvtqgkdi2ZJprB");
  console.log(" COLLECTION ---->", collection);
  console.log(("===================="));
  const Mint_a = new anchor.web3.PublicKey("EYogS25ACPuV9e7cd52JudFmciwuqLfTwBbMyeHDrkGe");
  console.log(" Mint A ---->", Mint_a);
  console.log(("===================="));
  const Mint_b = new anchor.web3.PublicKey("3d6vLfJmJAYB3kUWN5r2SiKGkifiF4abBSwo9bMVJG4b");
  console.log(" Mint B ---->", Mint_b);
  console.log(("===================="));
  const metadata_a = new anchor.web3.PublicKey("EVZ6ZiktJNt5Qr7S8QvSwsBmxM2383dHvkmUNAABECVR");
  console.log(" METADATA ---->", metadata_a);
  console.log(("===================="));
  const metadata_b = new anchor.web3.PublicKey("BrbD1FLbQETAWg21WtTny1EPT2Grh9KLpB3m26snrDXL");
  console.log(" METADATA ---->", metadata_b);
  console.log(("===================="));
  const TokenAddress_a = new anchor.web3.PublicKey("6R4je3NoiJvkD8h9HVnaqYfjsyNUVcnqMkVC2qhDvZyx");
  console.log(" TOKEN ADDRESS ---->", TokenAddress_a);
  console.log(("===================="));
  const TokenAddress_b = new anchor.web3.PublicKey("FFv3A6ibP5um4DVgxSujPPotsCyGwiabf865r1gLNYQo");
  console.log(" TOKEN ADDRESS ---->", TokenAddress_b);
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
    console.log("DELAGATED AUTHORITY PDA ---->", delegatedAuthPda);


    const [stakeVault] = await anchor.web3.PublicKey.findProgramAddressSync(
      [wallet.publicKey.toBuffer(),
      TokenAddress_a.toBuffer()],
      program.programId
    );
    console.log(("===================="));
    console.log("STAKE VAULT PDA ---->", stakeVault);

    //FIND PDA FOR METADATA A
    const metadataAddress_a = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        Mint_a.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )[0];
    console.log("====================");
    console.log(`metadata a initialized and its address ===> ${metadataAddress_a}`);


    //FIND PDA FOR METADATA B
    const metadataAddress_b = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        Mint_b.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    )[0];
    console.log("====================");
    console.log(`metadata b initialized and its address ===> ${metadataAddress_b}`);


    //FIND PDA FOR MASTER EDITION A
    const masterEditionAddress_a = (anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        Mint_a.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    ))[0];
    console.log("====================");
    console.log(
      `Master edition A initialized and its address ===> ${masterEditionAddress_a}`);

    //FIND PDA FOR MASTER EDITION B
    const masterEditionAddress_b = (anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        Mint_b.toBuffer(),
        Buffer.from("edition"),
      ],
      TOKEN_METADATA_PROGRAM_ID,
    ))[0];
    console.log("====================");
    console.log(
      `Master edition A initialized and its address ===> ${masterEditionAddress_b}`);




    try {
      let StakeIx = await program.methods
        .stakeSwap(Mint_a, Mint_b)
        .accounts({
          userA: wallet.publicKey,
          userB: user_b,
          nftBEdition: masterEditionAddress_b,
          nftBMint: Mint_b,
          nftBTokenAccount:TokenAddress_b ,
          metadataProgram: TOKEN_METADATA_PROGRAM_ID,
          nftAEdition: masterEditionAddress_a,
          nftAMint: Mint_a,
          nftATokenAccount: TokenAddress_a,
          programAuthority: delegatedAuthPda,
          stakeVault: stakeVault,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
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


















// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Stake } from "../target/types/stake";
// import {
//   ComputeBudgetProgram,
//   Connection, Keypair, LAMPORTS_PER_SOL,
//   PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram,
//   Transaction, TransactionInstruction,
//   sendAndConfirmRawTransaction,
//   sendAndConfirmTransaction
// } from "@solana/web3.js"

// import {
//   bundlrStorage,
//   keypairIdentity,
//   Metaplex,
// } from "@metaplex-foundation/js"
// import { TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddress } from "@solana/spl-token"




// describe("STAKE", () => {

//   console.log(`
//   ==============================
//   THE BEGINING OF DESCRIBE `);

//   const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
//   const ASSOCIATED_TOKEN_PROGRAM = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

//   const provider = anchor.AnchorProvider.env();
//   const wallet = provider.wallet as anchor.Wallet;
//   anchor.setProvider(anchor.AnchorProvider.env());
//   const walletPubKey = anchor.AnchorProvider.local().wallet.publicKey;
//   console.log(("===================="));
//   console.log(" WALLET PUBKEY ---->", walletPubKey);
//   console.log(("===================="));
//   const collection = new anchor.web3.PublicKey("G5WvFzffVU2vLW7Eitym5ebobmFAkXfvtqgkdi2ZJprB");
//   console.log(" COLLECTION ---->", collection);
//   console.log(("===================="));
//   const Mint = new anchor.web3.PublicKey("EYogS25ACPuV9e7cd52JudFmciwuqLfTwBbMyeHDrkGe");
//   console.log(" Mint ---->", Mint);
//   console.log(("===================="));
//   const METADATA = new anchor.web3.PublicKey("C5dYDC5oYYRr3sHprDRd9o9uc9Rm8A8zuZtRK8RZtXDE");
//   console.log(" METADATA ---->", METADATA);
//   console.log(("===================="));
//   const TokenAddress = new anchor.web3.PublicKey("4WqBecvoYktD34Pnowm3fWiGZBHE3nQ5j1MvqGmFskkN");
//   console.log(" TOKEN ADDRESS ---->", TokenAddress);
//   console.log(("===================="));
//   const program = anchor.workspace.Stake as Program<Stake>;
//   console.log("PROGRAM", program.programId.toBase58());




//   const BLOCKHASH = async () => {
//     const { blockhash, lastValidBlockHeight } = await program.provider.connection.getLatestBlockhash("finalized");
//     return {
//       blockhash: blockhash,
//       lastValidBlockHeight: lastValidBlockHeight
//     }
//   };

//   const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({
//     units: 1000000
//   });

//   const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({
//     microLamports: 1
//   });




//   it('IT STAKE', async () => {

//     console.log("THE BEGINING OF IT STAKE ...");


//     const [delegatedAuthPda] = await anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("authority")
//       ],
//       program.programId
//     );
//     console.log(("===================="));
//     console.log("DELAGATED AUTH PDA", delegatedAuthPda);


//     const [UserStakeInfoPda ] = await anchor.web3.PublicKey.findProgramAddressSync(
//       [wallet.publicKey.toBuffer(),
//       TokenAddress.toBuffer()],
//       program.programId
//     );
//     console.log(("===================="));
//     console.log("USER STAKE INFO PDA", UserStakeInfoPda);

//     //FIND PDA FOR METADATA
//     const metadataAddress = anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("metadata"),
//         TOKEN_METADATA_PROGRAM_ID.toBuffer(),
//         Mint.toBuffer(),
//       ],
//       TOKEN_METADATA_PROGRAM_ID
//     )[0];
//     console.log("====================");
//     console.log(`metadata initialized and its address ===> ${metadataAddress}`);


//     //FIND PDA FOR MASTER EDITION
//     const masterEditionAddress = (anchor.web3.PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("metadata"),
//         TOKEN_METADATA_PROGRAM_ID.toBuffer(),
//         Mint.toBuffer(),
//         Buffer.from("edition"),
//       ],
//       TOKEN_METADATA_PROGRAM_ID,
//     ))[0];
//     console.log("====================");
//     console.log(
//       `Master edition initialized and its address ===> ${masterEditionAddress}`);


//     // const Program_Authority: anchor.web3.PublicKey = delegatedAuthPda;
//     // let UserStakePda: anchor.web3.PublicKey = UserStakeInfoPda;
//     // let nft: any
//     // // let mintAuth: anchor.web3.PublicKey = ,
//     // let mint: anchor.web3.PublicKey = Mint;
//     // let NftAtokenAddress: anchor.web3.PublicKey = TokenAddress;
//     // let nftMasteredition: anchor.web3.PublicKey = masterEditionAddress;


//     try {
//       let StakeIx = await program.methods
//         .stakeSwap()
//         .accounts({
//           metadataProgram: TOKEN_METADATA_PROGRAM_ID,
//           nftAEdition: masterEditionAddress,
//           nftAMint: Mint,
//           nftATokenAccount: TokenAddress,
//           programAuthority: delegatedAuthPda,
//           stakeVault: UserStakeInfoPda,
//           systemProgram: SystemProgram.programId,
//           tokenProgram: TOKEN_PROGRAM_ID,
//           user: wallet.publicKey,
//         })
//         .signers([wallet.payer])
//         .instruction()

//       const StakeTx = new Transaction()
//         .add(addPriorityFee)
//         .add(modifyComputeUnits)
//         .add(StakeIx)
//       console.log("====================");
//       console.log("INSTRUCTIONS ADDED TO STAKE TX");

//       const blockhashData = await BLOCKHASH();
//       const { blockhash, lastValidBlockHeight } = blockhashData;
//       console.log("====================");
//       console.log("RECENT BLOCKHASH =====>", blockhash);
//       console.log("====================");
//       console.log("lastValidBlockHeight =====>", lastValidBlockHeight);

//       StakeTx.recentBlockhash = blockhash;
//       StakeTx.feePayer = wallet.publicKey;


//       try {

//         const signature = await sendAndConfirmTransaction(provider.connection, StakeTx, [wallet.payer]);
//         console.log("-----------------------");
//         console.log("SEND AND CONFIRM STAKE TRANSACTION SIGNATURE =====>", signature);



//         const confirmMintTx = await program.provider.connection.confirmTransaction({
//           blockhash,
//           lastValidBlockHeight,
//           signature,
//         });
//         console.log("-----------------------");
//         console.log("CONFIRM TRANSACTION =====>", confirmMintTx);




//         const result = await provider.connection.getParsedTransaction(signature, "confirmed");
//         console.log("-----------------------");
//         console.log("STAKE TX RESULT =====>", result);
//       } catch (Error) {
//         console.log("ERROR IN STAKE TRY TX");
//         console.error(Error);
//       }

//     } catch (Error) {
//       console.log(`STAKE ERROR IN BIG PICTURE OF STAKE ${Error}`);
//       console.error(Error)
//     }
//   });
// });
