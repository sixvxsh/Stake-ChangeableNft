use std::fs::Metadata;

use anchor_lang::{prelude::*, solana_program::stake::state::StakeState};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Approve, Mint, MintTo, Revoke, Token, TokenAccount},
};

use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};

use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::Space;


declare_id!("37HsMb2NSamepLG98j7MyYiB9E5tDBzsPYWVmoR32sJ2");

#[program]
pub mod stake {
    use super::*;

    pub fn stake_swap(
        _ctx: Context<Stake>, nft_name: String, wanted_nft: String) -> Result<()> {

            require!()

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> { 
    #[account(
        init_if_needed,
        payer = user,
        space = std::mem::size_of::<Vault>() + 8,
        seeds = [user.key().as_ref(), nft_token_account.key().as_ref()],
        bump
    )]
    pub stake_vault: Account<'info, Vault>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=user
    )]
    pub nft_token_account: Account<'info, TokenAccount>,
    pub nft_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    // pub metadata_program: Program<'info, Metadata>,

}


#[account]
pub struct Vault {
    user_pubkey: Pubkey,
    nft_token_account: Pubkey,
    stake_start: u64,
    is_initialize: bool,
    stake_state: StakeState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
#[derive(InitSpace)]
pub enum StakeState {
    Staked,
    Unstaked,
}






































































// use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, TokenAccount, Transfer};
// use solana_program::program::{invoke, invoke_signed};
// use solana_program::system_instruction;

// declare_id!("Your program ID");

// #[program]
// pub mod nft_staking {
//     use super::*;

//     pub fn stake_nft(ctx: Context<StakeNFT>, nft_name: String) -> ProgramResult {
//         // Implement the staking logic here
//         unimplemented!();
//     }

//     pub fn swap_nft(ctx: Context<SwapNFT>, new_nft_name: String) -> ProgramResult {
//         // Implement the swapping logic here
//         unimplemented!();
//     }
// }

// #[derive(Accounts)]
// pub struct StakeNFT<'info> {
//     #[account(mut, signer)]
//     pub staker: AccountInfo<'info>,
//     #[account(
//         init,
//         seeds = [b"nft_vault".as_ref()],
//         bump,
//         payer = staker,
//     )]
//     pub nft_vault: ProgramAccount<'info, NFTVault>,
//     #[account(mut)]
//     pub mint: AccountInfo<'info>,
//     #[account(mut)]
//     pub token_account: AccountInfo<'info>,
// }

// #[account]
// pub struct NFTVault {
//     pub name: String,
// }

// #[derive(Accounts)]
// pub struct SwapNFT<'info> {
//     #[account(mut, signer)]
//     pub swapper: AccountInfo<'info>,
//     #[account(mut)]
//     pub nft_vault: ProgramAccount<'info, NFTVault>,
//     #[account(mut)]
//     pub new_nft_account: AccountInfo<'info>,
// }

// impl<'info> From<&mut StakeNFT<'info>> for Transfer<'info> {
//     fn from(accs: &mut StakeNFT<'info>) -> Self {
//         Transfer {
//             from: accs.token_account.clone(),
//             to: accs.nft_vault.token_account.clone(),
//             authority: accs.staker.clone(),
//             amount: 1,
//         }
//     }
// }

// #[derive(Accounts)]
// pub struct MintToken<'info> {
//     #[account(mut)]
//     pub mint: AccountInfo<'info>,
//     #[account(init, payer = payer)]
//     pub token_account: AccountInfo<'info>,
//     #[account(signer)]
//     pub payer: AccountInfo<'info>,
// }

// #[derive(Accounts)]
// pub struct StakeVault<'info> {
//     #[account(init)]
//     pub vault: ProgramAccount<'info, NFTVault>,
//     pub rent: Sysvar<'info, Rent>,
// }

// impl<'info> StakeVault<'info> {
//     fn accounts(ctx: &Context<StakeVault<'info>>) -> Result<()> {
//         let vault = &mut ctx.accounts.vault;
//         vault.name = ctx.accounts.vault_name.to_string();

//         Ok(())
//     }
// }

// #[associated]
// impl<'info> StakeNFT<'info> {
//     pub fn Stake_vault(
//         ctx: Context<StakeVault<'info>>,
//         vault_name: String,
//     ) -> Result<()> {
//         let mut ctx = ctx.accounts;
//         ctx.vault_name = vault_name;
//         ctx
//     }

//     pub fn mint_token(
//         ctx: Context<MintToken<'info>>,
//         amount: u64,
//     ) -> Result<()> {
//         let mut ctx = ctx.accounts;
//         ctx.amount = amount;
//         ctx
//     }
// }

// #[cfg(not(feature = "program"))]
// mod tests {
//     use super::*;
//     use anchor_lang::prelude::*;
//     use anchor_spl::token::{self, Mint};

//     #[tokio::test]
//     async fn test_stake_nft() {
//         // Stake the test environment
//         let program_id = Pubkey::new_unique();
//         let mut test = program::ProgramTest::new("nft-staking", program_id, processor!(nft_staking::processor));
        
//         // Add program derived accounts
//         let mint_key = test.add_account(token::id(), 0, Mint {
//             mint_authority: COption::Some(test.payer.pubkey()),
//             supply: 0,
//             ..Mint::default()
//         });
//         let nft_vault_key = test.add_account(
//             to_account_info(&program_id, false),
//             0,
//             NFTVault {
//                 name: "Test NFT Vault".to_string(),
//             },
//         );

//         // Set the program authority
//         test.set_program_authority(program_id, None);

//         // Create the test environment
//         let (mut banks_client, payer, recent_blockhash) = test.start().await;

//         // TODO: Implement the test case
//     }
// }
