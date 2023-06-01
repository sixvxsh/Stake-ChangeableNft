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
   

    use super::*;

    pub fn stake_swap(ctx: Context<Stake>) -> Result<()> {

        
        // {{ FIRST SCENARIO }}

        // DO WE HAVE THE NFT REQUESTED FROM THE USER ?
        // NO

        // { FIRST PHASE } 
        //1- Take delegate of nft_A from user (to program)
        //2- Freeze authority of nft_A (to program)

        msg!("FIRST SCENARIO ");

        require!(
            ctx.accounts.stake_vault.stake_state == StakeState::Unstaked,
            StakeError::AlreadyStaked
        );

        if !ctx.accounts.stake_vault.is_initialize {
            ctx.accounts.stake_vault.is_initialize = true;
        }

        let clock = Clock::get()?;

        msg!("1- TAKE DELEGATE FOR NFT_A FROM USER ...");

        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = Approve {
            to: ctx.accounts.nft_a_token_account.to_account_info(),
            delegate: ctx.accounts.program_authority.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);
        token::approve(cpi_approve_ctx , 1)?;


        
        // msg!("SECOND SCENARIO - PHASE 1");
        let authority_bump = *ctx.bumps.get("program_authority").unwrap();
        msg!("2- FREEZE AUTRHORITY TO PROGRAM FOR NFT_A");
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
        );

        ctx.accounts.stake_vault.token_account = ctx.accounts.nft_a_token_account.key();
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
        seeds = [user.key().as_ref(), nft_a_token_account.key().as_ref()],
        bump,
        payer = user,
        space = 8 + UserStakeInfo::INIT_SPACE)]
    pub stake_vault: Account<'info, UserStakeInfo>,

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
    /// CHECK: We're about to create this with Metaplex
    #[account(owner=MetadataTokenId)]
    pub nft_a_edition: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub token_metadata_program: Program<'info, Metadata>,

    // pub nft_b_token_account: Account<'info, TokenAccount>,
}


#[account]
#[derive(InitSpace)]
pub struct UserStakeInfo {
    user_pubkey: Pubkey,
    token_account: Pubkey,
    stake_start: u64,
    is_initialize: bool,
    stake_state: StakeState,
}

#[derive(PartialEq, AnchorSerialize, AnchorDeserialize, Clone, Copy)]
#[derive(InitSpace)]
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

    //     msg!("FIRST SCENARIO - PHASE 1");
    //     msg!("1- TAKE DELEGATE FOR NFT_A FROM USER ");
        
    //     let cpi_approve_program =  ctx.accounts.token_program.to_account_info();
    //     let cpi_approve_accounts =  token::Approve {
    //         to: ctx.accounts.nft_a_token_account.to_account_info(),
    //         delegate: ctx.accounts.program_authority.to_account_info(),
    //         authority: ctx.accounts.user.to_account_info(),
    //     };
    //     let cpi_approve_ctx = CpiContext::new(cpi_approve_program , cpi_approve_accounts);

    //     token::approve(cpi_approve_ctx , 1);

    //     msg!("FIRST SCENARIO - PHASE 1");
    //     msg!("2- FREEZE AUTRHORITY TO PROGRAM FOR NFT_A ");
    //     invoke_signed(
    //         &freeze_delegated_account(
    //             ctx.accounts.token_program.key(), 
    //             ctx.accounts.program_authority.key(), 
    //             ctx.accounts.nft_a_token_account.key(), 
    //             ctx.accounts.nft_edition.key(), 
    //             ctx.accounts.nft_mint.key()
    //         ), 
    //         &[
    //             ctx.accounts.token_program.to_account_info(),
    //             ctx.accounts.nft_a_token_account.to_account_info(),
    //             ctx.accounts.program_authority.to_account_info(),
    //             ctx.accounts.nft_edition.to_account_info(),
    //             ctx.accounts.nft_mint.to_account_info(),
    //         ], 
    //         &[&[&[ctx.accounts.program_authority.key()]]],
    //     );

    //  // let nft_token_account = ctx.accounts.stake_vault.nft_token_account;

    //     msg!("FIRST SCENARIO - PHASE 2");
    //     msg!("1- UNFREEZE NFT_B DELEGATE FROM PROGRAM AUTHORITY");
    //     invoke_signed(
    //         &thaw_delegated_account(
    //             ctx.accounts.metadata_program.key(), 
    //             ctx.accounts.program_authority.key(), 
    //             ctx.accounts.nft_b_token_account.key(), 
    //             ctx.accounts.nft_edition.key(), 
    //             ctx.accounts.nft_mint.key()
    //         ),
    //         &[
    //             ctx.accounts.metadata_program.to_account_info(),
    //             ctx.accounts.nft_edition.to_account_info(),
    //             ctx.accounts.nft_mint.to_account_info(),
    //             ctx.accounts.nft_b_token_account.to_account_info(),
    //             ctx.accounts.program_authority.to_account_info()
    //         ], 
    //         &[&[&[signers]]]
    //     );

    //     msg!("FIRST SCENARIO - PHASE 2");
    //     msg!("2- TRANSFER DELEGATE OF THE NFT_B TO USER");

    //     let cpi_program2 = ctx.accounts.token_program.to_account_info();
    //     let cpi_accounts2 = Approve {
    //         to: ctx.accounts.nft_b_token_account.to_account_info(),
    //         delegate: ctx.accounts.user.to_account_info(),
    //         authority: ctx.accounts.program_authority.to_account_info(),
    //     };

    //     let cpi_approve2_ctx = CpiContext::new(cpi_program2, cpi_accounts2);
    //     token::approve(cpi_approve2_ctx , 1);

    //     msg!("FIRST SCENARIO - PHASE 2");
    //     msg!("3- FREEZ AUTHORITY NFT_B TO USER");
    //     invoke_signed(
    //         &freeze_delegated_account(
    //             ctx.accounts.metadata_program.key(), 
    //             ctx.accounts.user.key(), 
    //             ctx.accounts.nft_b_token_account.key(), 
    //             ctx.accounts.nft_edition.key(), 
    //             ctx.accounts.nft_mint.key()
    //         ), 
    //         &[
    //             ctx.accounts.metadata_program.to_account_info(),
    //             ctx.accounts.user.to_account_info(),
    //             ctx.accounts.nft_b_token_account.to_account_info(),
    //             ctx.accounts.nft_edition.to_account_info(),
    //             ctx.accounts.nft_mint.to_account_info(),
    //         ],
    //         &[&[&[Delegate]]]
    //     );

    //     msg!(" FIRST SCENARIO - PHASE 2");
    //     msg!("4- TRANSFER NFT_B TO USER WALLET");

    //     let cpi_transfer_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_transfer_accounts = token::Transfer {
    //         from: ctx.accounts.program_authority.to_account_info(),
    //         to: ctx.accounts.user.to_account_info(),
    //         authority: ctx.accounts.program_authority.to_account_info(),
    //     };
    //     let Cpi_Transfer_Ctx = CpiContext::new(cpi_transfer_program , cpi_transfer_accounts);

    //     token::transfer(Cpi_Transfer_Ctx , 1);
