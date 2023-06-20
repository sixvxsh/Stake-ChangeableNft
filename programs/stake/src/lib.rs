use std::vec;

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::{Approve, Mint, Token, TokenAccount};
use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};

use {
    anchor_lang::{prelude::*, solana_program::program::invoke, system_program},
    anchor_spl::associated_token,
    mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID},
    // mpl_token_metadata::instruction::
    // {create_master_edition_v3,
    // create_metadata_accounts_v3},
};
// use mpl_token_metadata::state::Metadata;
use anchor_lang::Space;

declare_id!("37HsMb2NSamepLG98j7MyYiB9E5tDBzsPYWVmoR32sJ2");

#[program]
pub mod stake {

    use super::*;

    pub fn stake_swap(ctx: Context<Stake>, _nft_a: Pubkey, _nft_b: Pubkey) -> Result<()> {
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

        let _clock = Clock::get()?;

        msg!("CREATING TOKEN ACCOUNT FOR {NFT_A_TREASURY} ACCOUNT...");
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.program_authority.to_account_info(),
                associated_token: ctx.accounts.nft_a_treasury_account.to_account_info(),
                authority: ctx.accounts.program_authority.to_account_info(),
                mint: ctx.accounts.nft_a_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;

        msg!("CREATING TOKEN ACCOUNT FOR {NFT_B_TREASURY} ACCOUNT...");
        associated_token::create(CpiContext::new(
            ctx.accounts.associated_token_program.to_account_info(),
            associated_token::Create {
                payer: ctx.accounts.program_authority.to_account_info(),
                associated_token: ctx.accounts.nft_b_treasury_account.to_account_info(),
                authority: ctx.accounts.program_authority.to_account_info(),
                mint: ctx.accounts.nft_b_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
            },
        ))?;


        // Transfer NFT_A from user A to program
        msg!("TRANSFERING NFT_A FROM USER A...");
        let cpi1_program = ctx.accounts.token_program.to_account_info();
        let cpi1_accounts = token::Transfer {
            from: ctx.accounts.nft_a_token_account.to_account_info(),
            to: ctx.accounts.nft_a_treasury_account.to_account_info(),
            authority: ctx.accounts.user_a.to_account_info(),
        };
        let token_transfer_context = CpiContext::new(cpi1_program, cpi1_accounts);
        token::transfer(token_transfer_context, 1)?;
        msg!("Transfered NFT_A from user A to program's treasury");

        // msg!("FIRST SCENARIO ");
        msg!("1-1 TAKE DELEGATE FOR NFT_A FROM USER ...");
        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = token::Approve {
            to: ctx.accounts.nft_a_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user_a.to_account_info(),
        };
        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        token::approve(cpi_approve_ctx, 1)?;

        // msg!("FIRST SCENARIO ");
        msg!("1-2 FREEZEING AUTRHORITY TO PROGRAM FOR NFT_A");
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
        msg!("NFT_A's AUTHORIUTY FREEZED");

        // Transfer NFT_B from user A to program
        msg!("TRANSFERING NFT_B FROM USER B...");
        let cpi2_program = ctx.accounts.token_program.to_account_info();
        let cpi2_accounts = token::Transfer {
            from: ctx.accounts.nft_b_token_account.to_account_info(),
            to: ctx.accounts.nft_b_treasury_account.to_account_info(),
            authority: ctx.accounts.user_b.to_account_info(),
        };
        let token_transfer_context = CpiContext::new(cpi2_program, cpi2_accounts);
        token::transfer(token_transfer_context, 1)?;
        msg!("Transfered NFT_B from USER_B to program's treasury ");

        msg!("1-3 TAKING DELEGATE FOR NFT_B FROM USER TO PROGRAM... ");
        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = token::Approve {
            to: ctx.accounts.nft_b_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user_b.to_account_info(),
        };
        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        token::approve(cpi_approve_ctx, 1)?;
        msg!("TAKED DELEGATE FOR NFT_B FROM USER B ");

        let authority_bump = *ctx.bumps.get("program_authority").unwrap();
        msg!("1-4 FREEZING AUTRHORITY OF NFT_B FROM USER TO PROGRAM... ");
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

        msg!("NFT_B's AUTHORIUTY FREEZED");

        // ctx.accounts.stake_vault.token_account_a = ctx.accounts.nft_a_token_account.key();
        // ctx.accounts.stake_vault.user_a_pubkey = ctx.accounts.user_a.key();
        // ctx.accounts.stake_vault.user_b_pubkey = ctx.accounts.user_b.key();
        // ctx.accounts.stake_vault.stake_state = StakeState::Staked;
        // ctx.accounts.stake_vault.stake_start = clock.unix_timestamp as u64;
        // ctx.accounts.stake_vault.is_initialize = true;
        // ctx.accounts.stake_vault.token_account_b = ctx.accounts.nft_b_token_account.key();
        // ctx.accounts.stake_vault.is_initialize = true;

        // let vault = &mut ctx.accounts.stake_vault;

        let _user_b = &mut ctx.accounts.user_b;

        // Get stake_vault account
        let stake_vault = &mut ctx.accounts.stake_vault;
        stake_vault.user_b_pubkey = ctx.accounts.user_b.key();

        msg!("TRANSFERING NFT A FROM (NFT_A_TREASURY_ACCOUNT) TO USER_B_NFT ACCOUNT..");
        let cpi3_transfer_program = ctx.accounts.token_program.to_account_info();
        let cpi3_transfer_accounts = token::Transfer {
            from: ctx.accounts.nft_a_treasury_account.to_account_info(),
            to: ctx.accounts.nft_b_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let signer: &[&[&[u8]]] = &[&[&b"authority"[..]]];
        let cpi_transfer_ctx =
            CpiContext::new_with_signer(cpi3_transfer_program, cpi3_transfer_accounts, signer);

        token::transfer(cpi_transfer_ctx, 1)?;
        msg!(" TRANSFERED NFT_A FROM PROGRAM'S TREASURY TO USER B WALLET");


        msg!("TRANSFERING NFT B FROM (NFT_B_TREASURY_ACCOUNT) TO USER_A_NFT ACCOUNT..");
        let cpi4_transfer_program = ctx.accounts.token_program.to_account_info();
        let cpi4_transfer_accounts = token::Transfer {
            from: ctx.accounts.nft_b_treasury_account.to_account_info(),
            to: ctx.accounts.nft_a_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(),
        };
        let signer: &[&[&[u8]]] = &[&[&b"authority"[..]]];
        let cpi_transfer_ctx =
            CpiContext::new_with_signer(cpi4_transfer_program, cpi4_transfer_accounts, signer);

        token::transfer(cpi_transfer_ctx, 1)?;
        msg!(" TRANSFERED NFT_B FROM PROGRAM'S TREASURY TO USER A WALLET");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(
        init_if_needed,
        seeds = [user_a.key().as_ref(), nft_a_token_account.key().as_ref()],
        bump,
        payer = user_a,
        space = 8 + StakeVault::INIT_SPACE)]
    pub stake_vault: Box<Account<'info, StakeVault>>,

    /// CHECK: Manual validation
    #[account(mut , seeds= ["authority".as_bytes().as_ref()] , bump)]
    pub program_authority: UncheckedAccount<'info>,

    /// CHECK: Manual validation
    #[account(mut)]
    pub user_b: AccountInfo<'info>,

    #[account(mut)]
    pub user_a: Signer<'info>,

    #[account(
        mut,
        associated_token::mint=nft_a_mint,
        associated_token::authority=user_a
    )]
    pub nft_a_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint=nft_b_mint,
        associated_token::authority=user_b
    )]
    pub nft_b_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user_a, // If init required, payer will be user
        associated_token::mint = mint, // If init required, mint will be set to Mint
        associated_token::authority = program_authority // If init required, authority set to PDA
    )]
    pub nft_a_treasury_account: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user_a, // If init required, payer will be user
        associated_token::mint = mint, // If init required, mint will be set to Mint
        associated_token::authority = program_authority // If init required, authority set to PDA
    )]
    pub nft_b_treasury_account: Box<Account<'info, TokenAccount>>,
    pub nft_a_mint: Account<'info, Mint>,
    pub nft_b_mint: Account<'info, Mint>,
    /// CHECK: We're about to create this with Metaplex
    #[account(owner=MetadataTokenId)]
    pub nft_a_edition: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(owner=MetadataTokenId)]
    pub nft_b_edition: UncheckedAccount<'info>,
    // ATA Program required to create ATA for nft_treasury_accounts
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub mint: Box<Account<'info, Mint>>,
}

#[account]
#[derive(InitSpace)]
pub struct StakeVault {
    pub user_a_pubkey: Pubkey,
    pub user_b_pubkey: Pubkey,
    pub token_account_a: Pubkey,
    pub token_account_b: Pubkey,
    pub stake_start: u64,
    pub is_initialize: bool,
    pub stake_state: StakeState,
    pub mint_nfts: Pubkey,
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

/////////////////////////////////////////////////////////////////////////////

// msg!("SECOND SCENARIO - PHASE 2");
// let nft_b: Pubkey = ctx.accounts.stake_vault.,
// msg!("2-1 UNFREEZING AUTHORITY OF NFT_B FROM PROGRAM AUTHORITY");
// invoke_signed(
//     &thaw_delegated_account(
//         ctx.accounts.metadata_program.key(),
//         ctx.accounts.program_authority.key(),
//         ctx.accounts.nft_b_token_account.key(),
//         ctx.accounts.nft_b_edition.key(),
//         ctx.accounts.nft_b_mint.key(),
//     ),
//     &[
//         ctx.accounts.metadata_program.to_account_info(),
//         ctx.accounts.nft_b_edition.to_account_info(),
//         ctx.accounts.nft_b_mint.to_account_info(),
//         ctx.accounts.nft_b_token_account.to_account_info(),
//         ctx.accounts.program_authority.to_account_info(),
//     ],
//     &[&[b"authority", &[authority_bump]]],
// )?;
// msg!(" UNFREEZED AUTHORITY OF NFT_B FROM PROGRAM AUTHORITY");

// msg!("SECOND SCENARIO - PHASE 2");
// msg!("2-2 TAKING DELEGATE OF NFT_B FROM PROGRAM TO USER B...");

// let cpi_approve_to_user_program = ctx.accounts.token_program.to_account_info();
// let cpi_approve_to_user_accounts = token::Approve {
//     to: ctx.accounts.nft_b_token_account.to_account_info(),
//     delegate: ctx.accounts.user_b.to_account_info(),
//     authority: ctx.accounts.program_authority.to_account_info(),
// };
// let cpi_approve2_ctx =
//     CpiContext::new(cpi_approve_to_user_program, cpi_approve_to_user_accounts);
// token::approve(cpi_approve2_ctx, 1)?;
// msg!(" TAKED DELEGATE OF NFT_B FROM PROGRAM TO USER_B...");

// // msg!("SECOND SCENARIO - PHASE 2");
// msg!("2-3 FREEZING AUTHORITY OF NFT_B TO USER_B ...");
// invoke_signed(
//     &freeze_delegated_account(
//         ctx.accounts.metadata_program.key(),
//         ctx.accounts.user_b.key(),
//         ctx.accounts.nft_b_token_account.key(),
//         ctx.accounts.nft_b_edition.key(),
//         ctx.accounts.nft_b_mint.key(),
//     ),
//     &[
//         ctx.accounts.metadata_program.to_account_info(),
//         ctx.accounts.user_b.to_account_info(),
//         ctx.accounts.nft_b_token_account.to_account_info(),
//         ctx.accounts.nft_b_edition.to_account_info(),
//         ctx.accounts.nft_b_mint.to_account_info(),
//     ],
//     &[&[b"authority", &[authority_bump]]],
// )?;
// msg!("FREEZED AUTHORITY OF NFT_B TO USER.");

// // msg!(" SECOND SCENARIO - PHASE 2");
// msg!("4- TRANSFERING NFT_B FROM PROGRAM TO USER WALLET ...");
