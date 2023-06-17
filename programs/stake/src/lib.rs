use std::vec;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::token;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Approve, Mint, MintTo, Revoke, Token, TokenAccount},
};
use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};
// use mpl_token_metadata::state::Metadata;
use anchor_lang::Space;

declare_id!("37HsMb2NSamepLG98j7MyYiB9E5tDBzsPYWVmoR32sJ2");

#[program]
pub mod stake {

    use super::*;

    pub fn stake_swap(ctx: Context<Stake> , nft_a: Pubkey , nft_b: Pubkey) -> Result<()> {
        // {{ FIRST SCENARIO }}

        // DO WE HAVE THE NFT REQUESTED FROM THE USER ?
        // NO

        // { FIRST PHASE }
        //1- Take delegate of nft_A from user (to program)
        //2- Freeze authority of nft_A (to program)

        // require!(
        //     ctx.accounts.stake_vault.stake_state == StakeState::Unstaked,
        //     StakeError::AlreadyStaked
        // );

        

        if !ctx.accounts.stake_vault.is_initialize {
            ctx.accounts.stake_vault.is_initialize = true;
        }

        let clock = Clock::get()?;
        msg!("FIRST SCENARIO ");
        msg!("1- TAKE DELEGATE FOR NFT_A FROM USER ...");

        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = Approve {
            to: ctx.accounts.nft_a_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        token::approve(cpi_approve_ctx, 1)?;


        
        msg!("FIRST SCENARIO ");
        msg!("2- FREEZE AUTRHORITY TO PROGRAM FOR NFT_A");
        let authority_bump = *ctx.bumps.get("program_authority").unwrap();
        invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.metadata_program.key(),
                ctx.accounts.program_authority.key(),
                ctx.accounts.nft_a_token_account.key(),
                ctx.accounts.nft_a_edition.key(),
                ctx.accounts.nft_a_mint.key(),
            ),
            &[
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_a_mint.to_account_info(),
                ctx.accounts.nft_a_token_account.to_account_info(),
                ctx.accounts.nft_a_edition.to_account_info(),
                ctx.accounts.metadata_program.to_account_info(),
            ],
            &[&[b"authority", &[authority_bump]]],
        )?;

        msg!("NFT_A AUTHORIUTY FREEZED");

        

        //-------

        msg!("SECOND SCENARIO - PHASE 1");
        msg!("1- TAKING DELEGATE FOR NFT_B FROM USER TO PROGRAM... ");

        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = token::Approve {
            to: ctx.accounts.nft_b_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        token::approve(cpi_approve_ctx, 1)?;
        msg!("TAKED DELEGATE FOR NFT_B FROM USER... ");


        msg!("SECOND SCENARIO - PHASE 1");
        let authority_bump = *ctx.bumps.get("program_authority").unwrap();
        msg!("2- FREEZING AUTRHORITY OF NFT_B FROM USER TO PROGRAM... ");
        invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.metadata_program.key(),
                ctx.accounts.program_authority.key(),
                ctx.accounts.nft_b_token_account.key(),
                ctx.accounts.nft_b_edition.key(),
                ctx.accounts.nft_b_mint.key(),
            ),
            &[
                ctx.accounts.metadata_program.to_account_info(),
                ctx.accounts.nft_b_token_account.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
                ctx.accounts.nft_b_edition.to_account_info(),
                ctx.accounts.nft_b_mint.to_account_info(),
            ],
            &[&[b"authority", &[authority_bump]]],
        )?;

        msg!("2- FREEZED AUTRHORITY OF NFT_B FROM USER TO PROGRAM. ");
        
        //-----

        ctx.accounts.stake_vault.token_account_a = ctx.accounts.nft_a_token_account.key();
        ctx.accounts.stake_vault.users_pubkey = ctx.accounts.user.key();
        ctx.accounts.stake_vault.stake_state = StakeState::Staked;
        ctx.accounts.stake_vault.stake_start = clock.unix_timestamp as u64;
        ctx.accounts.stake_vault.is_initialize = true;
        ctx.accounts.stake_vault.token_account_b = ctx.accounts.nft_b_token_account.key();
        ctx.accounts.stake_vault.is_initialize = true;

        ////////////////////////////////////////////////////////////////////////////////////

        // {{ SECOND SCENARIO }}

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

        // msg!("SECOND SCENARIO - PHASE 1");
        // msg!("1- TAKING DELEGATE FOR NFT_A FROM USER TO PROGRAM... ");

        // let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        // let cpi_approve_accounts = token::Approve {
        //     to: ctx.accounts.nft_a_token_account.to_account_info(),
        //     delegate: ctx.accounts.program_authority.to_account_info(),
        //     authority: ctx.accounts.user.to_account_info(),
        // };
        // let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        // token::approve(cpi_approve_ctx, 1)?;
        // msg!("TAKED DELEGATE FOR NFT_A FROM USER... ");


        // msg!("SECOND SCENARIO - PHASE 1");
        // msg!("2- FREEZING AUTRHORITY OF NFT_A FROM USER TO PROGRAM... ");
        // invoke_signed(
        //     &freeze_delegated_account(
        //         ctx.accounts.token_program.key(),
        //         ctx.accounts.program_authority.key(),
        //         ctx.accounts.nft_a_token_account.key(),
        //         ctx.accounts.nft_a_edition.key(),
        //         ctx.accounts.nft_a_mint.key(),
        //     ),
        //     &[
        //         ctx.accounts.token_program.to_account_info(),
        //         ctx.accounts.nft_a_token_account.to_account_info(),
        //         ctx.accounts.program_authority.to_account_info(),
        //         ctx.accounts.nft_a_edition.to_account_info(),
        //         ctx.accounts.nft_a_mint.to_account_info(),
        //     ],
        //     &[&[b"authority", &[authority_bump]]],
        // )?;
        // msg!("2- FREEZED AUTRHORITY OF NFT_A FROM USER TO PROGRAM. ");


        // let vault = &mut ctx.accounts.stake_vault;

        msg!("SECOND SCENARIO - PHASE 2");
        // let nft_b: Pubkey = ctx.accounts.stake_vault.,  
        msg!("1- UNFREEZING AUTHORITY OF NFT_B FROM PROGRAM AUTHORITY");
        invoke_signed(
            &thaw_delegated_account(
                ctx.accounts.metadata_program.key(),
                ctx.accounts.program_authority.key(),
                ctx.accounts.nft_b_token_account.key(),
                ctx.accounts.nft_b_edition.key(),
                ctx.accounts.nft_b_mint.key(),
            ),
            &[
                ctx.accounts.metadata_program.to_account_info(),
                ctx.accounts.nft_b_edition.to_account_info(),
                ctx.accounts.nft_b_mint.to_account_info(),
                ctx.accounts.nft_b_token_account.to_account_info(),
                ctx.accounts.program_authority.to_account_info(),
            ],
            &[&[b"authority", &[authority_bump]]],
        )?;
        msg!(" UNFREEZED AUTHORITY OF NFT_B FROM PROGRAM AUTHORITY");

        msg!("SECOND SCENARIO - PHASE 2");
        msg!("2- TAKING DELEGATE OF NFT_B FROM PROGRAM TO USER...");

        let cpi_approve_to_user_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_to_user_accounts = Approve {
            to: ctx.accounts.nft_b_token_account.to_account_info(),
            delegate: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let cpi_approve2_ctx =
            CpiContext::new(cpi_approve_to_user_program, cpi_approve_to_user_accounts);
        token::approve(cpi_approve2_ctx, 1)?;
        msg!("2- TAKED DELEGATE OF NFT_B FROM PROGRAM TO USER...");


        msg!("SECOND SCENARIO - PHASE 2");
        msg!("3- FREEZING AUTHORITY OF NFT_B TO USER ...");
        invoke_signed(
            &freeze_delegated_account(
                ctx.accounts.metadata_program.key(),
                ctx.accounts.user.key(),
                ctx.accounts.nft_b_token_account.key(),
                ctx.accounts.nft_b_edition.key(),
                ctx.accounts.nft_b_mint.key(),
            ),
            &[
                ctx.accounts.metadata_program.to_account_info(),
                ctx.accounts.user.to_account_info(),
                ctx.accounts.nft_b_token_account.to_account_info(),
                ctx.accounts.nft_b_edition.to_account_info(),
                ctx.accounts.nft_b_mint.to_account_info(),
            ],
            &[&[b"authority", &[authority_bump]]],
        )?;
        msg!("FREEZED AUTHORITY OF NFT_B TO USER.");

        msg!(" SECOND SCENARIO - PHASE 2");
        msg!("4- TRANSFERING NFT_B FROM PROGRAM TO USER WALLET ...");

        let cpi_transfer_program = ctx.accounts.token_program.to_account_info();
        let cpi_transfer_accounts = token::Transfer {
            from: ctx.accounts.program_authority.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let cpi_transfer_ctx = CpiContext::new(cpi_transfer_program, cpi_transfer_accounts);
        token::transfer(cpi_transfer_ctx, 1)?;
        msg!(" TRANSFERED NFT_B FROM PROGRAM TO USER WALLET");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        init_if_needed,
        seeds = [user.key().as_ref(), nft_a_token_account.key().as_ref()],
        bump,
        payer = user,
        space = 8 + StakeVault::INIT_SPACE)]
    pub stake_vault: Account<'info, StakeVault>,
    /// CHECK: Manual validation
    #[account(mut , seeds= ["authority".as_bytes().as_ref()] , bump)]
    pub program_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        associated_token::mint=nft_a_mint,
        associated_token::authority=user
    )]
    pub nft_a_token_account: Account<'info, TokenAccount>,
    pub nft_a_mint: Account<'info, Mint>,
    pub nft_b_mint: Account<'info, Mint>,
    /// CHECK: We're about to create this with Metaplex
    #[account(owner=MetadataTokenId)]
    pub nft_a_edition: UncheckedAccount<'info>,

    #[account(owner=MetadataTokenId)]
    pub nft_b_edition: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    // pub token_metadata_program: Program<'info, Metadata>,
    pub nft_b_token_account: Account<'info, TokenAccount>,
}

#[account]
#[derive(InitSpace)]
pub struct StakeVault {
    pub users_pubkey: Pubkey,
    pub token_account_a: Pubkey,
    pub token_account_b: Pubkey,
    pub stake_start: u64,
    pub is_initialize: bool,
    pub stake_state: StakeState,
    pub mint_nfts: Pubkey,
    pub edition_nft: Pubkey,
    pub nft_b: Pubkey,
}

#[derive(PartialEq, AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub enum StakeState {
    Staked,
    Unstaked,
}

#[error_code]
pub enum StakeError {
    #[msg("NFT already staked")]
    AlreadyStaked,

    #[msg("State account is uninitialized")]
    UninitializedAccount,

    #[msg("Stake state is invalid")]
    InvalidStakeState,
}

#[derive(Clone)]
pub struct Metadata;

impl anchor_lang::Id for Metadata {
    fn id() -> Pubkey {
        MetadataTokenId
    }
}
















        // ctx.accounts.stake_vault.mint_nfts = vec![]; 
        // vault.edition_nft = vec![];
        // vault.mint_nfts = vec![];
        // vault.users_pubkey = vec![];
        // vault.token_account_a = vec![];

        // vault.mint_nfts.push(ctx.accounts.nft_a_mint.key());
        // vault.edition_nft.push(ctx.accounts.nft_a_edition.key());
        // vault.users_pubkey.push(ctx.accounts.user.key());
        // vault.token_account_a.push(ctx.accounts.nft_a_token_account.key());
        

            // let nft_b = &mut ctx.accounts.stake_vault.nft_b;
        // // ctx.accounts.stake_vault.nft_b = nft_b;
        // // let nft_b_wanted = ctx.accounts.stake_vault.nft_b;

        // for account in &vault.mint_nfts {
        //     if nft_b == nft_b {
        //         //yes we have nft_wanted
        //     }
        //     else {
        //         // no we haven't
        //     }
        // }

        // i'm looking for these three accounts in three vector array!
        // let desired_account_a = ctx.accounts.nft_a_mint.key();
        // let desired_account_b = ctx.accounts.nft_a_edition.key();
        // let desired_account_c = ctx.accounts.nft_a_token_account.key();

        // now i want to retrieve them by index of them in their arrays:
        // let index_a = vault.mint_nfts.iter().position(|&account| account == desired_account_a );
        // let index_b = vault.edition_nft.iter().position(|&account| account == desired_account_b);
        // let index_c = vault.token_account_a.iter().position(|&account| account == desired_account_c);


        // if let Some(index_a) = index_a {
        //     let account_a = vault.mint_nfts[index_a];
        //     // Do something with account_a
        // }






        // let nft_token_account = ctx.accounts.stake_vault.nft_token_account;
        // let  ctx.accounts.nft_b_mint = nft_b;

        //wanted nft from user is {nft_b} and if its mint exist in mint_nfts array:

        // if in [mint_nfts] exist nft_b mint then:

        // retrieve nft_b_mint in [mint_nfts]
        // retrieve nft_b_token_account in [token_account]
        // retrieve nft_b_edition in [edition_nft]