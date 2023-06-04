import {
    bundlrStorage,
    keypairIdentity,
    Metaplex,
  } from "@metaplex-foundation/js"
  import { createMint, getAssociatedTokenAddress } from "@solana/spl-token"
  import * as anchor from "@coral-xyz/anchor"
import { Keypair } from "@solana/web3.js"
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes"
import { Stake } from "../../target/types/stake"
import { Program } from "@coral-xyz/anchor"
  
const payer =   Keypair.fromSecretKey(
      bs58.decode(
        "2BaV11dsovKiJ6uazisbn6RD7CEcbLQSrAjdieNr4Jw2BjwqjiJgRhotPVfVm3vuwSxMCJQGoibBknJv7SQE5BQ9"
      )
    )

const program = anchor.workspace.Stake as Program<Stake>;


const setupNft = async (program, payer) => {
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
      
  
    console.log("nft metadata pubkey: ", nft.metadataAddress.toBase58());
    console.log("nft token address: ", nft.tokenAddress.toBase58());

    const [delegatedAuthPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId
    );
    const [UserStakeInfoPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [payer.publicKey.toBuffer(), nft.tokenAddress.toBuffer()],
      program.programId
    );

    console.log("delegated authority pda: ", delegatedAuthPda.toBase58());
    console.log("stake state pda: ", UserStakeInfoPda.toBase58());
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
      stakeStatePda: UserStakeInfoPda,
      mint: mint,
      mintAuth: mintAuth,
      tokenAddress: tokenAddress,
    }
  }

setupNft(payer , program);
