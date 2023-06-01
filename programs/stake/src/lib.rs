use std::fs::Metadata;

use anchor_lang::{prelude::*, solana_program::stake::state::StakeState as OtherStakeState};

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

use anchor_spl::{
    associated_token,
    token,
    
};


declare_id!("37HsMb2NSamepLG98j7MyYiB9E5tDBzsPYWVmoR32sJ2");

#[program]
pub mod stake {
    use anchor_lang::{solana_program::example_mocks::solana_sdk::signers, system_program::Transfer};

    use super::*;

    pub fn stake_swap(ctx: Context<Stake>, nft_name: String, nft_req: String) -> Result<()> {


        if !ctx.accounts.stake_vault.is_initialize {
            ctx.accounts.stake_vault.is_initialize = true;
        }

        require!(ctx.accounts.stake_vault.stake_state == StakeState::Unstaked,
            //Error
        );
        let clock = Clock::get().unwrap();


        // {{ FIRST SCENARIO }}

        // DO WE HAVE THE NFT REQUESTED FROM THE USER ?
        // YES

        // { First Phase }
        //1- Take delegate of nft_A from user (to program)
        //2- Freeze authority of nft_A (to program)
        

        // { Second Phase }
        //1- unfreeze nft_B delagte from program_authority
        //2- transfer the nft_B delegate to user (approve delegate)
        //3- freeze authority of nft_B (to user)
        //3- transfer nft_B to user wallet

        msg!("FIRST SCENARIO - PHASE 1");
        msg!("1- TAKE DELEGATE FOR NFT_A FROM USER ");
        
        let cpi_approve_program =  ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts =  token::Approve {
            to: ctx.accounts.token_account_nft_a.to_account_info(),
            delegate: ctx.program_authority.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_approve_ctx = CpiContext::new(cpi_approve_program , cpi_approve_accounts);

        token::approve(cpi_approve_ctx , 1);

        msg!("FIRST SCENARIO - PHASE 1");
        msg!("2- FREEZE AUTRHORITY TO PROGRAM FOR NFT_A ");
        invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.token_program.key(), 
                ctx.accounts.program_authority.key(), 
                ctx.accounts.token_account_nft_a.key(), 
                ctx.accounts.nft_edition.key(), 
                ctx.accounts.nft_mint.key()
            ), 
            &[
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.token_account_nft_a.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
            ], 
            &[&[&[signers]]],
        );

     // let nft_token_account = ctx.accounts.stake_vault.nft_token_account;

        msg!("FIRST SCENARIO - PHASE 2");
        msg!("1- UNFREEZE NFT_B DELEGATE FROM PROGRAM AUTHORITY");
        invoke_signed(
            &thaw_delegated_account(
                ctx.accounts.metadata_program.key(), 
                ctx.accounts.program_authority.key(), 
                ctx.accounts.token_account_nft_b.key(), 
                ctx.accounts.nft_edition.key(), 
                ctx.accounts.nft_mint.key()
            ),
            &[
                ctx.accounts.metadata_program.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
                ctx.accounts.token_account_nft_b.to_account_info(),
                ctx.accounts.program_authority.to_account_info()
            ], 
            &[&[&[signers]]]
        );

        msg!("FIRST SCENARIO - PHASE 2");
        msg!("2- TRANSFER DELEGATE OF THE NFT_B TO USER");

        let cpi_program2 = ctx.accounts.token_program.to_account_info();
        let cpi_accounts2 = Approve {
            to: ctx.accounts.token_account_nft_b.to_account_info(),
            delegate: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };

        let cpi_approve2_ctx = CpiContext::new(cpi_program2, cpi_accounts2);
        token::approve(cpi_approve2_ctx , 1);

        msg!("PHASE 2");
        msg!("3- FREEZ AUTHORITY NFT_B TO USER");
        invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.metadata_program.key(), 
                ctx.accounts.user.key(), 
                ctx.accounts.token_account_nft_b.key(), 
                ctx.accounts.nft_edition.key(), 
                ctx.accounts.nft_mint.key()
            ), 
            &[
                ctx.accounts.metadata_program.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.nft_req.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
            ],
            &[&[&[signers]]]
        );

        msg!(" FIRST SCENARIO - PHASE 2");
        msg!("4- TRANSFER NFT_B TO USER WALLET");

        let cpi_transfer_program = ctx.accounts.token_program.to_account_info();
        let cpi_transfer_accounts = token::Transfer {
            from: ctx.accounts.program_authority.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let Cpi_Transfer_Ctx = CpiContext::new(cpi_transfer_program , cpi_transfer_accounts);

        token::transfer(Cpi_Transfer_Ctx , 1);

////////////////////////////////////////////////////////////////////////////////////////////////////////////
        
        // {{ SECOND SCENARIO }}

        // DO WE HAVE THE NFT REQUESTED FROM THE USER ?
        // NO

        // { FIRST PHASE } 
        //1- Take delegate of nft_A from user (to program)
        //2- Freeze authority of nft_A (to program)

        msg!("SECOND SCENARIO - PHASE 1");
        msg!("1- TAKE DELEGATE FOR NFT_A FROM USER ");

        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = Approve {
            to: ctx.accounts.nftA_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        token::approve(cpi_approve_ctx , 1);



        let authority_bump = *ctx.bumps.get("program_authority").unwrap();
        msg!("SECOND SCENARIO - PHASE 1");
        msg!("2- FREEZE AUTRHORITY TO PROGRAM FOR NFT_A");
        invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.metadata_program.key(), 
                ctx.accounts.program_authority.key(),
                ctx.accounts.nftA_token_account.key(),
                ctx.accounts.nft_edition.key(),
                ctx.accounts.nft_mint.key(),
            ),
            &[
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_mint.to_account_info(),
                ctx.accounts.nftA_token_account.to_account_info(),
                ctx.accounts.nft_edition.to_account_info(),
                ctx.accounts.metadata_program.to_account_info(),
            ],
            &[&[b"authority", &[authority_bump]]],
        );

        ctx.accounts.stake_vault.token_account = ctx.accounts.nft_token_account.key();
        ctx.accounts.stake_vault.user_pubkey = ctx.accounts.user.key();
        ctx.accounts.stake_vault.stake_state = StakeState::Staked;
        ctx.accounts.stake_vault.stake_start = clock.unix_timestamp;
               

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

    #[account(mut , seeds= ["authority".as_bytes().as_ref()] , bump)]
    pub program_authority: UncheckedAccount<'info>,


    pub nft_edition: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_metadata_program: Program<'info, Metadata>,
}


#[account]
#[derive(InitSpace)]
pub struct Vault {
    user_pubkey: Pubkey,
    token_account: Pubkey,
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
