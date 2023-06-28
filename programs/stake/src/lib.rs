use std::vec;

use anchor_lang::{prelude::*, solana_program};
use anchor_lang::solana_program::program::invoke_signed;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token::{Approve, Mint, Token, TokenAccount};
use mpl_token_metadata::{
    instruction::{freeze_delegated_account, thaw_delegated_account},
    ID as MetadataTokenId,
};

use anchor_lang::system_program;

// use {
//     anchor_lang::{prelude::*, solana_program::program::invoke, system_program},
//     anchor_spl::associated_token,
//     mpl_token_metadata::{instruction as token_instruction, ID as TOKEN_METADATA_ID},
//     // mpl_token_metadata::instruction::
//     // {create_master_edition_v3,
//     // create_metadata_accounts_v3},
// };
// use mpl_token_metadata::state::Metadata;
use anchor_lang::Space;

declare_id!("37HsMb2NSamepLG98j7MyYiB9E5tDBzsPYWVmoR32sJ2");

#[program]
pub mod stake {

    use mpl_token_metadata::pda::find_program_as_burner_account;

    use super::*;

    pub fn initialize(ctx: Context<Initialize> ) -> Result<()> {
        let stake_swap_authority = &mut ctx.accounts.stake_swap_authority;
        msg!("Stake Swap Authority Account Created");
        Ok(())
    }

    pub fn stake(ctx: Context<Stake> ) -> Result<()> {


        let authority_bump = *ctx.bumps.get("nft_vault").unwrap();

 
        let seeds = &[
            b"stake1".as_ref(),
            &ctx.accounts.nft_mint.key().to_bytes(),
            &[authority_bump],
        ];

        let signer = &[&seeds[..]];

    

        msg!("1 TAKEING DELEGATE FOR NFT FROM USER ...");
        let cpi_approve_program = ctx.accounts.token_program.to_account_info();
        let cpi_approve_accounts = token::Approve {
            to: ctx.accounts.nft_token_account.to_account_info(),
            delegate: ctx.accounts.nft_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };

        let cpi_approve_ctx = CpiContext::new(cpi_approve_program, cpi_approve_accounts);

        token::approve(cpi_approve_ctx, 1)?;
        msg!("2 TAKED DELEGATE FROM USER");


        // Transfer NFT_A from user A to program
        msg!("nft_a_token_account: {}", &ctx.accounts.nft_token_account.key());
        msg!("nft_a_treasury_token_account: {}", &ctx.accounts.nft_treasury_token_account.key());
        msg!("authority for transfer: {}", &ctx.accounts.nft_vault.key());


        msg!("TRANSFERING NFT FROM USER ...");
        let cpi1_program = ctx.accounts.token_program.to_account_info();
        let cpi1_accounts = token::Transfer {
            from: ctx.accounts.nft_token_account.to_account_info(),
            to: ctx.accounts.nft_treasury_token_account.to_account_info(),
            authority: ctx.accounts.nft_vault.to_account_info(),
        };

        let token_transfer_context = CpiContext::new_with_signer(cpi1_program, cpi1_accounts, signer);

        token::transfer(token_transfer_context, 1)?;

        msg!("Transfered NFT_A from user A to program's treasury");

        // ctx.accounts.nft_vault.token_account = ctx.accounts.nft_treasury_token_account.key();
        ctx.accounts.nft_vault.mint_nft = ctx.accounts.nft_mint.key();
        ctx.accounts.nft_vault.user_pubkey = ctx.accounts.user.key();


        // msg!("TREANSFERING 1 SOL");

        // msg!("TRANSFER FROM: {}", &ctx.accounts.user.key());
        // msg!("TRANSFER TO: {}", &ctx.accounts.sol_treasury.key());

        // let cpi_sol_context = CpiContext::new(
        //     ctx.accounts.system_program.to_account_info(), 
        //     system_program::Transfer {
        //         from: ctx.accounts.user.to_account_info(),
        //         to: ctx.accounts.sol_treasury.to_account_info(),
        //     });
        // system_program::transfer(cpi_sol_context, 1000000000)?;

        // msg!("TRANSFERED 1SOL TO SOL TREASURY");

        Ok(())

    }


    pub fn swap(ctx: Context<Swap> ) -> Result<()> {


        msg!("BEGINING OF SWAP IX");

        let authority_bump = *ctx.bumps.get("nft_vault").unwrap();
        msg!("Authority bump: {}" , authority_bump );


        let authority_bump = *ctx.bumps.get("nft_vault").unwrap();
 
        let seeds = &[
            b"stake1".as_ref(),
            &ctx.accounts.nft_mint.key().to_bytes(),
            &[authority_bump],
        ];

        let nft_vault = &[&seeds[..]];



        msg!("TRANSFERING NFT..");

       
        msg!("nft_treasury_token_account: {}", &ctx.accounts.nft_treasury_token_account.key());
        
        msg!("nft_token_swap_account: {}", &ctx.accounts.nft_token_swap_account.key());

        msg!("Transfer_authority: {}", &ctx.accounts.nft_vault.key());




        let cpi3_transfer_program = ctx.accounts.token_program.to_account_info();
        let cpi3_transfer_accounts = token::Transfer {
            from: ctx.accounts.nft_treasury_token_account.to_account_info(),
            to: ctx.accounts.nft_token_swap_account.to_account_info(),
            authority: ctx.accounts.nft_vault.to_account_info(),
        };

        let cpi_transfer_ctx =
            CpiContext::new_with_signer(cpi3_transfer_program, cpi3_transfer_accounts, nft_vault);

        token::transfer(cpi_transfer_ctx, 1)?;

        msg!("TRANSFERED NFT..");







        ///////////////////////////////////////////////////////////////////////////



        
        // msg!("TRANSFERING NFT_A1 IN VAULT FROM (NFT_TREASURY_TOKEN_ACCOUNT) TO USER_B1_NFT SWAP ACCOUNT..");
        // let cpi3_transfer_program = ctx.accounts.token_program.to_account_info();
        // let cpi3_transfer_accounts = token::Transfer {
        //     from: ctx.accounts.nft_treasury_token_account.to_account_info(),
        //     to: ctx.accounts.nft_user_b1_token_swap_account.to_account_info(),
        //     authority: ctx.accounts.nft_vault.to_account_info(),
        // };

        // let cpi_transfer_ctx =
        //     CpiContext::new_with_signer(cpi3_transfer_program, cpi3_transfer_accounts, nft_vault);

        // token::transfer(cpi_transfer_ctx, 1)?;
        // msg!(" TRANSFERED NFT_A1 TO USER_B WALLET");



        // msg!("nft_b_treasury_account: {}", &ctx.accounts.nft_b_treasury_account.key());
        // msg!("nft_a_token_account: {}", &ctx.accounts.nft_a_swap_account.key());
        // msg!("authority for transfer: {}", &ctx.accounts.stake_swap_authority.key());

        // msg!("TRANSFERING NFT_A2 IN VAULT FROM (NFT_TREASURY_TOKEN_ACCOUNT) TO USER_B2_NFT SWAP ACCOUNT..");
        // let cpi4_transfer_program = ctx.accounts.token_program.to_account_info();
        // let cpi4_transfer_accounts = token::Transfer {
        //     from: ctx.accounts.nft_treasury_token_account.to_account_info(),
        //     to: ctx.accounts.nft_user_b2_token_swap_account.to_account_info(),
        //     authority: ctx.accounts.nft_vault.to_account_info(),
        // };


        // let cpi_transfer_ctx =
        //     CpiContext::new_with_signer(cpi4_transfer_program, cpi4_transfer_accounts, signer);

        // token::transfer(cpi_transfer_ctx, 1)?;
        // msg!(" TRANSFERED NFT_B FROM PROGRAM'S TREASURY TO USER A WALLET");






        // msg!("TRANSFERING NFT_A3 IN VAULT FROM (NFT_TREASURY_TOKEN_ACCOUNT) TO USER_B3_NFT SWAP ACCOUNT..");
        // let cpi4_transfer_program = ctx.accounts.token_program.to_account_info();
        // let cpi4_transfer_accounts = token::Transfer {
        //     from: ctx.accounts.nft_treasury_token_account.to_account_info(),
        //     to: ctx.accounts.nft_user_b3_token_swap_account.to_account_info(),
        //     authority: ctx.accounts.nft_vault.to_account_info(),
        // };


        // let cpi_transfer_ctx =
        //     CpiContext::new_with_signer(cpi4_transfer_program, cpi4_transfer_accounts, signer);

        // token::transfer(cpi_transfer_ctx, 1)?;
        // msg!(" TRANSFERED NFT_B FROM PROGRAM'S TREASURY TO USER A WALLET");




        Ok(())

    }

}

#[derive(Accounts)]
pub struct Initialize <'info> {

    /// CHECK: Manual validation
    #[account(
        init,
        payer = owner, 
        space = 8 + StakeVault::INIT_SPACE,
        seeds = [b"authority".as_ref(),
        owner.key().as_ref(),
        ],
        bump,
        )]
    pub stake_swap_authority: Box<Account<'info , StakeVault>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info , System>,
}

#[derive(Accounts)]
pub struct Stake<'info> {

    /// CHECK: Manual validation
    // #[account(
    //     mut,
    //     seeds = [b"authority".as_ref(), owner.key().as_ref()],
    //     bump,
    //     )]
    // pub stake_swap_authority: Box<Account<'info , StakeVault>>,


    /// CHECK: Manual validation
    // #[account(mut)]
    // pub owner: Signer<'info>,


    /// CHECK: Manual validation
    #[account(mut)]
    pub user: Signer<'info>,


    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub nft_token_account: Box<Account<'info, TokenAccount>>,


    #[account(
        init,
        payer = user, 
        associated_token::mint = nft_mint, 
        associated_token::authority = nft_vault 
    )]
    pub nft_treasury_token_account: Box<Account<'info, TokenAccount>>,


    #[account(
        init,
        payer = user,
        space = 8 + NftVault::INIT_SPACE,
        seeds = [b"stake1".as_ref() , nft_mint.key().as_ref()],
        bump
    )]
    pub nft_vault: Box<Account<'info, NftVault>>,

    /// CHECK: Manual validation
    #[account(mut)]
    pub sol_treasury: AccountInfo<'info>,


    pub nft_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub metadata_program: Program<'info, Metadata>,

}

#[derive(Accounts)]
pub struct Swap<'info> {

    /// CHECK: Manual validation
    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Manual validation
    #[account(mut)]
    pub user_swap: AccountInfo<'info>,


    #[account(
        mut,
        seeds = [b"stake1".as_ref() , nft_mint.key().as_ref()],
        bump
    )]
    pub nft_vault: Box<Account<'info, NftVault>>,



    #[account(
        mut, 
        associated_token::mint = nft_mint, 
        associated_token::authority = nft_vault 
    )]
    pub nft_treasury_token_account: Box<Account<'info, TokenAccount>>,


    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = user
    )]
    pub nft_token_account: Box<Account<'info, TokenAccount>>,




    #[account(
        init,
        payer = user,
        associated_token::mint = nft_mint,
        associated_token::authority = user_swap

    )]
    pub nft_token_swap_account: Box<Account<'info, TokenAccount>>,


    pub nft_mint: Account<'info, Mint>,


    // ATA Program required to create ATA for nft_treasury_accounts
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub metadata_program: Program<'info, Metadata>,




    //// CHECK: Manual validation
    // #[account(
    //     mut,
    //     seeds = ["authority".as_bytes().as_ref(),
    //     user_a.key().as_ref() ,
    //     nft_a_mint.key().as_ref(),
    //     nft_b_mint.key().as_ref()],
    //     // bump = stake_swap_authority.bump,
    //     bump
    //     )]
    // pub stake_swap_authority: Account<'info , StakeVault>,



    // #[account(
    //     mut,
    //     associated_token::mint = nft_a_mint, 
    //     associated_token::authority = stake_swap_authority,
    // )]
    // pub nft_a_treasury_account: Box<Account<'info, TokenAccount>>,



    // #[account(
    //     mut,
    //     associated_token::mint = nft_b_mint, 
    //     associated_token::authority = stake_swap_authority, 

    // )]
    // pub nft_b_treasury_account: Box<Account<'info, TokenAccount>>,

    






    // #[account(
    //     init,
    //     payer = user_a,
    //     associated_token::mint = nft_a1_mint,
    //     associated_token::authority = user_b

    // )]
    // pub nft_user_b1_token_swap_account: Box<Account<'info, TokenAccount>>,




    // #[account(
    //     init,
    //     payer = user_a,
    //     associated_token::mint = nft_a2_mint,
    //     associated_token::authority = user_b

    // )]
    // pub nft_user_b2_token_swap_account: Box<Account<'info, TokenAccount>>,





    // #[account(
    //     init,
    //     payer = user_a,
    //     associated_token::mint = nft_a3_mint,
    //     associated_token::authority = user_b

    // )]
    // pub nft_user_b3_token_swap_account: Box<Account<'info, TokenAccount>>,


    // #[account(
    //     init,
    //     payer = user_a,
    //     associated_token::mint = nft_a_mint,
    //     associated_token::authority = user_b

    // )]
    // pub nft_b_swap_account: Box<Account<'info, TokenAccount>>,


    // pub nft_a1_mint: Account<'info, Mint>,
    // pub nft_a2_mint: Account<'info, Mint>,
    // pub nft_a3_mint: Account<'info, Mint>,

    // pub nft_b_mint: Account<'info, Mint>,


}




#[account]
#[derive(InitSpace)]
pub struct StakeVault {
    pub bump: u8 ,
    pub token_account: Pubkey,
    pub mint_nft: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct NftVault {
    pub bump: u8,
    pub token_account: Pubkey,
    pub mint_nft: Pubkey,
    pub date: u64,
    pub user_pubkey: Pubkey,
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



        // let seeds = &[
        //     b"stake_info".as_ref(),
        //     &ctx.accounts.mint_authority.key().to_bytes(),
        //     &ctx.accounts.mint.key().to_bytes(),
        //     &[auth_bump],
        // ];

        // let signer = &[&seeds[..]];











        // msg!("CREATING TOKEN ACCOUNT FOR {NFT_A_TREASURY} ACCOUNT...");
        // msg!("nft_a_treasury_account: {}", &ctx.accounts.nft_a_treasury_account.key());
        // msg!("payer: {}", &ctx.accounts.program_authority.key());
        // msg!("mint a: {}", &ctx.accounts.nft_a_mint.key());
        // associated_token::create(
        //     CpiContext::new(
        //     ctx.accounts.associated_token_program.to_account_info(),
        //     associated_token::Create {
        //         payer: ctx.accounts.user_a.to_account_info(),
        //         associated_token: ctx.accounts.nft_a_treasury_account.to_account_info(),
        //         authority: ctx.accounts.user_a.to_account_info(),
        //         mint: ctx.accounts.nft_a_mint.to_account_info(),
        //         system_program: ctx.accounts.system_program.to_account_info(),
        //         token_program: ctx.accounts.token_program.to_account_info(),
        //     },
        // ))?;
        // msg!("ASSOCIATED A CREATED");

        // msg!("CREATING TOKEN ACCOUNT FOR {NFT_B_TREASURY} ACCOUNT...");
        // msg!("nft_b_treasury_account: {}", &ctx.accounts.nft_b_treasury_account.key());
        // msg!("payer: {}", &ctx.accounts.program_authority.key());
        // msg!("mint b: {}", &ctx.accounts.nft_b_mint.key());
        // associated_token::create(
        //     CpiContext::new(
        //     ctx.accounts.associated_token_program.to_account_info(),
        //     associated_token::Create {
        //         payer: ctx.accounts.user_b.to_account_info(),
        //         associated_token: ctx.accounts.nft_b_token_account.to_account_info(),
        //         authority: ctx.accounts.user_b.to_account_info(),
        //         mint: ctx.accounts.nft_b_mint.to_account_info(),
        //         system_program: ctx.accounts.system_program.to_account_info(),
        //         token_program: ctx.accounts.token_program.to_account_info(),
        //     },
        // ))?;
        // msg!("ASSOCIATED B CREATED");



        // let signer: &[&[&[u8]]] = &[&[&b"authority"[..]]];
        // &[&[b"authority", &[authority_bump]]],






        // ctx.accounts.stake_vault.token_account_a = ctx.accounts.nft_a_token_account.key();
        // ctx.accounts.stake_vault.user_a_pubkey = ctx.accounts.user_a.key();
        // ctx.accounts.stake_vault.user_b_pubkey = ctx.accounts.user_b.key();
        // ctx.accounts.stake_vault.stake_state = StakeState::Staked;
        // ctx.accounts.stake_vault.stake_start = clock.unix_timestamp as u64;
        // ctx.accounts.stake_vault.is_initialize = true;
        // ctx.accounts.stake_vault.token_account_b = ctx.accounts.nft_b_token_account.key();
        // ctx.accounts.stake_vault.is_initialize = true;






    //     if **profile_pda_account.try_borrow_lamports()? > 0 {
    //         msg!("This account has been initialized");
    //         initialized = true;
    //  } else {
    //         msg!("Account is not initialized");
    //         initialized = false;
    //  }