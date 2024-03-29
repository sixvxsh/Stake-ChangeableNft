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
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";





describe("STAKE-SWAP", () => {

  console.log(`
  ==============================
  THE BEGINING OF DESCRIBE `);

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const ASSOCIATED_TOKEN_PROGRAM = new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

  const provider = anchor.AnchorProvider.env();

  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(anchor.AnchorProvider.env());

  const programOwner = anchor.AnchorProvider.local().wallet.publicKey;
  console.log(("===================="));
  console.log(" WALLET PUBKEY(USER 1) ---->", programOwner);




  // const user_b_keypair = Keypair.fromSecretKey(
  //   Uint8Array.from([
  //     148, 220, 74, 86, 68, 72, 55, 74, 186, 215,
  //     223, 48, 99, 219, 180, 164, 171, 193, 88, 243,
  //     177, 43, 122, 156, 4, 162, 36, 208, 148, 118,
  //     102, 186, 234, 250, 119, 2, 109, 159, 135, 132,
  //     83, 102, 140, 128, 240, 204, 106, 254, 143, 207,
  //     169, 205, 250, 127, 158, 204, 12, 161, 221, 26,
  //     217, 245, 90, 233
  //   ])
  // );

  const user_b_keypair = Keypair.fromSecretKey(
    bs58.decode(
      "3ycw215tSDyHFbcm6s22RdH2si8YBXGjuRtrGArdCyGUaKAiv3dq2zWFuwVvTnMbvG1S9MB62QumdimJpGp2EBSY"
    )
  );

  console.log(("===================="));
  const user_b = new anchor.web3.PublicKey("GpFuzeBf6oQm98fiTr375rb8HfmgjQA2nU7CwZgG7dtC");
  console.log(" user_b ---->", user_b);

  // console.log(("===================="));
  // const collection = new anchor.web3.PublicKey("G5WvFzffVU2vLW7Eitym5ebobmFAkXfvtqgkdi2ZJprB");
  // console.log(" COLLECTION ---->", collection);


  console.log(("===================="));
  const sol_treasury_keypair = Keypair.fromSecretKey(
    bs58.decode(
      "2t369UEFHJStbE6Z99Z5MjtBef6eD7pUaDMPtppzLtEFx3MUHYjj2TmD1irPB2pyBRjpKyaqo14pjV3yk8LgkVca"
    )
  );


  console.log(("===================="));
  const sol_treasury_pub = new anchor.web3.PublicKey("84xk4xYQcrYVekAcciFE1bKWDsp7L5tJfvr59ToDRoca");


  /////////////////////////////////////////  MINT ACCOUNT's //////////////////////////////////////////////

  console.log(("===================="));
  const mint = new anchor.web3.PublicKey("6fpMdSdfGKE4LmkWnTtvemnQi1a2HkzeX96qTdgEUfFq");
  console.log(" Mint  ---->", mint);



  // console.log(("===================="));
  // const mint_b = new anchor.web3.PublicKey("8AF86ar2FEFcutx1jZzGFLeWkfJWAdx9mswVXD5kPmuw");
  // console.log(" Mint B ---->", mint_b);



  // const metadata_a = new anchor.web3.PublicKey("EVZ6ZiktJNt5Qr7S8QvSwsBmxM2383dHvkmUNAABECVR");
  // console.log(" METADATA ---->", metadata_a);
  // console.log(("===================="));


  // const metadata_b = new anchor.web3.PublicKey("ELdQDmHa9k7yKeyfXwzFrxvFnrtfX4RnrjNqqzQBhHyW");
  // console.log(" METADATA ---->", metadata_b);
  // console.log(("===================="));

  //////////////////////////////////// TOKEN ACCOUNTs ////////////////////////////////

  const TokenAccount = new anchor.web3.PublicKey("CecUyGzrugTCTw37xpfoZHob3FYixuWTSoExoa1jWSFa");
  console.log(" TOKEN ADDRESS A ---->", TokenAccount);
  console.log(("===================="));


  //////////////////////////////////////////////////////////////////

  const token_address_for_swap = new anchor.web3.PublicKey("CEMwjQou1R9DCCPUVvBi51gbLwhJ3ahm9HTXTxd6EfHM");
  console.log(" TOKEN ADDRESS FOR SWAP A ---->", token_address_for_swap);
  console.log(("===================="));


  // const token_address_for_swap_b = new anchor.web3.PublicKey("EQhRnLe8DvPdsxP9xVHdU8JY17uqKwcwtkXof271xgGa");
  // console.log(" TOKEN ADDRESS FOR SWAP B ---->", token_address_for_swap_b);
  // console.log(("===================="));


  const program = anchor.workspace.Stake as Program<Stake>;
  console.log("PROGRAM ADDRESS", program.programId.toBase58());




  /////////////////////////// PDAs /////////////////////


  // const [StakeSwapAuth, authority] = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     // anchor.utils.bytes.utf8.encode("authority"),
  //     Buffer.from("authority"),
  //     provider.wallet.publicKey.toBuffer(),
  //   ],
  //   program.programId
  // );

  // console.log(("===================="));
  // console.log(" STAKE SWAP AUTHORITY ---->", StakeSwapAuth);


  const [nft_vault, stake1] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      // anchor.utils.bytes.utf8.encode("authority"),
      Buffer.from("stake1"),
      mint.toBuffer(),
    ],
    program.programId
  );

  console.log(("===================="));
  console.log(" NFT VAULT PDA ---->", nft_vault);


 

  // console.log(`Reviewer: `, pdadata.toString());
  // console.log(`Restaurant: `, pdadata.);
  // console.log(`Review: `, pdadata.review);
  // console.log(`Rating: `, pdadata.rating);


  ///////////////////////// TREASURY ACCOUNTs /////////////////////

  const nft_treasury_token_account = anchor.utils.token.associatedAddress({
    mint: mint,
    owner: nft_vault
  });
  console.log("-----------------------");
  console.log(`TREASURY A Token Address (ATA) ===> ${nft_treasury_token_account}`);








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


  // it('IT STAKE', async () => {

  //   console.log("THE BEGINING OF IT STAKE ...");

  //   try {
  //     let StakeIx = await program.methods
  //       .stake()
  //       .accounts({
  //         nftMint: mint,
  //         nftTokenAccount: TokenAccount,
  //         nftTreasuryTokenAccount: nft_treasury_token_account,
  //         nftVault: nft_vault,
  //         solTreasury: sol_treasury_pub,
  //         user: user_b,
  //         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
  //         metadataProgram: TOKEN_METADATA_PROGRAM_ID,
  //         systemProgram: SystemProgram.programId,
  //         tokenProgram: TOKEN_PROGRAM_ID,
  //       })
  //       .signers([user_b_keypair])
  //       .instruction()

  //     const StakeTx = new Transaction()
  //       .add(addPriorityFee)
  //       .add(modifyComputeUnits)
  //       .add(StakeIx)
  //     console.log("====================");
  //     console.log("INSTRUCTIONS ADDED TO STAKE TX");

  //     const blockhashData = await BLOCKHASH();
  //     const { blockhash, lastValidBlockHeight } = blockhashData;
  //     console.log("====================");
  //     console.log("RECENT BLOCKHASH =====>", blockhash);
  //     console.log("====================");
  //     console.log("lastValidBlockHeight =====>", lastValidBlockHeight);

  //     StakeTx.recentBlockhash = blockhash;
  //     StakeTx.feePayer = wallet.publicKey;


  //     try {
  //       // for sending this transaction the wallet.payer needed
  //       const signature = await sendAndConfirmTransaction(provider.connection, StakeTx, [wallet.payer , user_b_keypair]);
  //       console.log("-----------------------");
  //       console.log("SEND AND CONFIRM STAKE TRANSACTION SIGNATURE =====>", signature);



  //       const confirmMintTx = await program.provider.connection.confirmTransaction({
  //         blockhash,
  //         lastValidBlockHeight,
  //         signature,
  //       });
  //       console.log("-----------------------");
  //       console.log("CONFIRM TRANSACTION =====>", confirmMintTx);


  //       const result = await provider.connection.getParsedTransaction(signature, "confirmed");
  //       console.log("-----------------------");
  //       console.log("STAKE TX RESULT =====>", result);
  //     } catch (Error) {
  //       console.log("ERROR IN STAKE TRY TX");
  //       console.error(Error);
  //     }

  //   } catch (Error) {
  //     console.log(`STAKE ERROR IN BIG PICTURE OF STAKE ${Error}`);
  //     console.error(Error)
  //   }
  // });



  it("FETCH PDA" , async () => {
    const pdadata = await program.account.nftVault.fetch(nft_vault);
    console.log("PDA DATA", pdadata);
  });


  it("IT SWAP", async () => {
    console.log("THE BEGINING OF IT SWAP ...");
    try {
      let SwapIx = await program.methods
        .swap()
        .accounts({
          // solTreasury: sol_treasury_pub,
          nftMint: mint,
          nftTokenSwapAccount: token_address_for_swap,
          nftTreasuryTokenAccount: nft_treasury_token_account,
          nftVault: nft_vault,
          user: wallet.publicKey,
          userReceiver: user_b,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM,
          metadataProgram: TOKEN_METADATA_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
        })
        .signers([ wallet.payer , user_b_keypair  ])
        .instruction()

      const SwapTx = new Transaction()
        .add(addPriorityFee)
        .add(modifyComputeUnits)
        .add(SwapIx)
      console.log("====================");
      console.log("INSTRUCTIONS ADDED TO SWAP TX");

      const blockhashData = await BLOCKHASH();
      const { blockhash, lastValidBlockHeight } = blockhashData;
      console.log("====================");
      console.log("RECENT BLOCKHASH =====>", blockhash);
      console.log("====================");
      console.log("lastValidBlockHeight =====>", lastValidBlockHeight);

      SwapTx.recentBlockhash = blockhash;
      SwapTx.feePayer = wallet.publicKey;


      try {

        const signature = await sendAndConfirmTransaction(provider.connection, SwapTx, [wallet.payer, user_b_keypair , sol_treasury_keypair]);
        console.log("-----------------------");
        console.log("SEND AND CONFIRM SWAP TRANSACTION SIGNATURE =====>", signature);


        const confirmMintTx = await program.provider.connection.confirmTransaction({
          blockhash,
          lastValidBlockHeight,
          signature,
        });
        console.log("-----------------------");
        console.log("CONFIRM TRANSACTION =====>", confirmMintTx);


        const result = await provider.connection.getParsedTransaction(signature, "confirmed");
        console.log("-----------------------");
        console.log("SWAP TX RESULT =====>", result);
      } catch (Error) {
        console.log("ERROR IN SWAP TRY TX");
        console.error(Error);
      }

    } catch (Error) {
      console.log(`SWAP ERROR IN BIG TRY-CATCH OF SWAP ${Error}`);
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







//////////////////////////////////////////////////



//FIND PDA FOR METADATA NFT A
// const metadataAddress_a = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("metadata"),
//     TOKEN_METADATA_PROGRAM_ID.toBuffer(),
//     mint_a.toBuffer(),
//   ],
//   TOKEN_METADATA_PROGRAM_ID
// )[0];
// console.log("====================");
// console.log(`metadata a initialized and its address ===> ${metadataAddress_a}`);




//FIND PDA FOR METADATA NFT B
// const metadataAddress_b = anchor.web3.PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("metadata"),
//     TOKEN_METADATA_PROGRAM_ID.toBuffer(),
//     mint_b.toBuffer(),
//   ],
//   TOKEN_METADATA_PROGRAM_ID
// )[0];
// console.log("====================");
// console.log(`metadata b initialized and its address ===> ${metadataAddress_b}`);





//FIND PDA FOR MASTER EDITION A
// const masterEditionAddress_a = (anchor.web3.PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("metadata"),
//     TOKEN_METADATA_PROGRAM_ID.toBuffer(),
//     mint_a.toBuffer(),
//     Buffer.from("edition"),
//   ],
//   TOKEN_METADATA_PROGRAM_ID,
// ))[0];
// console.log("====================");
// console.log(
//   `Master edition A initialized and its address ===> ${masterEditionAddress_a}`);




//FIND PDA FOR MASTER EDITION B
// const masterEditionAddress_b = (anchor.web3.PublicKey.findProgramAddressSync(
//   [
//     Buffer.from("metadata"),
//     TOKEN_METADATA_PROGRAM_ID.toBuffer(),
//     mint_b.toBuffer(),
//     Buffer.from("edition"),
//   ],
//   TOKEN_METADATA_PROGRAM_ID,
// ))[0];
// console.log("====================");
// console.log(
//   `Master edition A initialized and its address ===> ${masterEditionAddress_b}`);






// const stakeswapSeeds = [
//   provider.wallet.publicKey.toBuffer(),
//   Buffer.from(anchor.utils.bytes.utf8.encode("authority")),
//   mint_a.toBuffer(),
//   mint_b.toBuffer(),
// ];



// const [StakeSwapAuth, _] = anchor.web3.PublicKey.findProgramAddressSync(
//   stakeswapSeeds,
//   program.programId
// );


