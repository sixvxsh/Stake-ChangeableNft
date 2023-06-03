import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stake } from "../target/types/stake";
import {
  ComputeBudgetProgram, 
  Connection, Keypair, LAMPORTS_PER_SOL,
  PublicKey, SYSVAR_RENT_PUBKEY, SystemProgram,
  Transaction, TransactionInstruction, 
  sendAndConfirmRawTransaction, 
  sendAndConfirmTransaction} from "@solana/web3.js"

import {
  bundlrStorage,
  keypairIdentity,
  Metaplex,
} from "@metaplex-foundation/js"
import { createMint, getAssociatedTokenAddress } from "@solana/spl-token"







describe("stake", () => {

  console.log(`
  ==============================
  THE BEGINING OF DESCRIBE `);

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
  const ASSOCIATED_TOKEN_PROGRAM =  new anchor.web3.PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;
  anchor.setProvider(anchor.AnchorProvider.env());
  const walletPubKey = anchor.AnchorProvider.local().wallet.publicKey;
  console.log(("===================="));
  console.log(" WALLET PUBKEY ---->" , walletPubKey);

  const program = anchor.workspace.Stake as Program<Stake>;


  const MintNft = async (program, payer) => {
    const metaplex = Metaplex.make(program.provider.connection)
      .use(keypairIdentity(payer))
      .use(bundlrStorage())
  
    const nft = await metaplex
      .nfts()
      .create({
        uri: "https://storage.googleapis.com/fractal-launchpad-public-assets/honeyland/assets_gold_pass/57.json",
        name: "Gold Pass #057",
        sellerFeeBasisPoints: 0,
      })
      
  
    console.log("nft metadata pubkey: ", nft.metadataAddress.toBase58())
    console.log("nft token address: ", nft.tokenAddress.toBase58())
    const [delegatedAuthPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId
    )
    const [UserStakeInfoPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [payer.publicKey.toBuffer(), nft.tokenAddress.toBuffer()],
      program.programId
    )
  
    console.log("delegated authority pda: ", delegatedAuthPda.toBase58())
    console.log("stake state pda: ", UserStakeInfoPda.toBase58())
    const [mintAuth] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("mint")],
      program.programId
    )
  
    const mint = await createMint(
      program.provider.connection,
      payer,
      mintAuth,
      null,
      2
    )
    console.log("Mint pubkey: ", mint.toBase58())
  
    const tokenAddress = await getAssociatedTokenAddress(mint, payer.publicKey)
  
    return {
      nft: nft,
      delegatedAuthPda: delegatedAuthPda,
      UserStakeInfoPda: UserStakeInfoPda,
      mint: mint,
      mintAuth: mintAuth,
      tokenAddress: tokenAddress,
    }
  }







 

  const BLOCKHASH = async () => {
    const { blockhash, lastValidBlockHeight } = await program.provider.connection.getLatestBlockhash("finalized");

    return {
      blockhash: blockhash,
      lastValidBlockHeight: lastValidBlockHeight
    }
    // console.log("-----------------------");
    // console.log("RECENT BLOCKHASH =====>" , blockhash );
    // console.log("-----------------------");
    // console.log( "lastValidBlockHeight =====>", lastValidBlockHeight);
  };


  const modifyComputeUnits = ComputeBudgetProgram.setComputeUnitLimit({ 
    units: 1000000 
  });

  const addPriorityFee = ComputeBudgetProgram.setComputeUnitPrice({ 
    microLamports: 1 
  });



   
  const TokenAddress = anchor.utils.token.associatedAddress({
    mint: MintKey.publicKey,
    owner: wallet.publicKey
  });
  console.log("-----------------------");
  console.log(`Token Address (ATA) address ===> ${TokenAddress}`);


  //FIND PDA FOR METADATA
  const metadataAddress = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      MintKey.publicKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  )[0];
  console.log("-----------------------");
  console.log(`metadata initialized and its address ===> ${metadataAddress}`);

  //FIND PDA FOR MASTER EDITION
  const masterEditionAddress = (anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      MintKey.publicKey.toBuffer(),
      Buffer.from("edition"),
    ],
    TOKEN_METADATA_PROGRAM_ID,
  ))[0];
  console.log("-----------------------");
  console.log(
    `Master edition metadata initialized and its address ===> ${masterEditionAddress}`);


  it('IT STAKE' ,  async => {



    try {

      const StakeIx = program.methods
      .stakeSwap()
      .accounts({
        metadataProgram:,
        nftAEdition: ,
        nftAMint: ,
        nftATokenAccount: ,
        programAuthority: ,
        stakeVault: ,
        systemProgram: ,
        tokenProgram: ,
        user: ,
      })
      .signers([user])
      .instruction()
    
    } catch (error) {
      
    }








  })
 
  








});
