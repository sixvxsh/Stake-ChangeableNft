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


  const Mintnft = async (program, payer) => {
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


    console.log("nft METADATA PUBKEY: ", nft.metadataAddress.toBase58());
    console.log("nft A TOKEN ADDRESS: ", nft.tokenAddress.toBase58());


    const [Program_Authority] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId
    );

    const [UserStakeInfoPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [payer.publicKey.toBuffer(), nft.tokenAddress.toBuffer()],
      program.programId
    );

    console.log("PROGRAM AUTHORITY PDA: ", Program_Authority.toBase58());
    console.log("USER STAKE INFO PDA: ", UserStakeInfoPda.toBase58());


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

    const NftAtokenAddress = await getAssociatedTokenAddress(
      mint,
      payer.publicKey)

    console.log("-----------------------");
    console.log(`nft A Token Address (ATA) address ===> ${NftAtokenAddress}`);

    return {
      nft: nft,
      Program_Authority: Program_Authority,
      UserStakeInfoPda: UserStakeInfoPda,
      mint: mint,
      mintAuth: mintAuth,
      NftAtokenAddress: NftAtokenAddress,
    }
  };

  Mintnft(program , wallet);
  console.log("MINT nft FUNCTION CALLED");
  

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





  // //FIND PDA FOR METADATA
  // const metadataAddress = anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("metadata"),
  //     TOKEN_METADATA_PROGRAM_ID.toBuffer(),
  //     MintKey.publicKey.toBuffer(),
  //   ],
  //   TOKEN_METADATA_PROGRAM_ID
  // )[0];
  // console.log("-----------------------");
  // console.log(`metadata initialized and its address ===> ${metadataAddress}`);

  //FIND PDA FOR MASTER EDITION
  // const masterEditionAddress = (anchor.web3.PublicKey.findProgramAddressSync(
  //   [
  //     Buffer.from("metadata"),
  //     TOKEN_METADATA_PROGRAM_ID.toBuffer(),
  //     MintKey.publicKey.toBuffer(),
  //     Buffer.from("edition"),
  //   ],
  //   TOKEN_METADATA_PROGRAM_ID,
  // ))[0];
  // console.log("-----------------------");
  // console.log(
  //   `Master edition metadata initialized and its address ===> ${masterEditionAddress}`);


  let Program_Authority: anchor.web3.PublicKey
  let UserStakeInfoPda: anchor.web3.PublicKey
  let nft: any
  let mintAuth: anchor.web3.PublicKey
  let mint: anchor.web3.PublicKey
  let NftAtokenAddress: anchor.web3.PublicKey


  it('IT STAKE', async => {

    try {
      const StakeIx = program.methods
        .stakeSwap()
        .accounts({
          metadataProgram:TOKEN_METADATA_PROGRAM_ID,
          nftAEdition: nft.masterEditionAddress,
          nftAMint: nft.mint,
          nftATokenAccount: NftAtokenAddress,
          programAuthority: Program_Authority ,
          stakeVault:UserStakeInfoPda,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          user: wallet.publicKey,
        })
        .signers([wallet.payer])
        .instruction()

    } catch (error) {
      
    }








  })










});
