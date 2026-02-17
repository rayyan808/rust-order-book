use anchor_lang::prelude::*;
use super::shared::transfer_tokens;

use crate::{error::ErrorCode, state::Offer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
/**
 * This struct represents the inputs a user will provide
 * when they want to make an offer. #macros are used to perform
 * validation on the inputs, under the hood these call
 * code in Anchor that does the validation for us, reducing our boilerplate
 */
#[derive(Accounts)] //We want to use Accounts from Anchor to help validate inputs
pub struct MakeOffer<'info> {
    //Used to find/create token accounts based off (wallet, token)
    associated_token_program: Program<'info, AssociatedToken>,

    //Used to execute functions (transfer, burn, mint etc..)
    token_program: Interface<'info, TokenInterface>,

    //Used to create account ()
    system_program: Program<'info, System>,

    #[account(mut, signer)] //We will modify the makers account by transferring from them
    maker: Signer<'info>, //Cleaner way of doing #[accounts(signer)]

    //Define the two tokens the user swaps
    //Ensure both tokens were created by the same token_program
    //specifically, by the token_program the user has specified in `token_program`
    #[account(mint::token_program = token_program)]
    token_mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    token_mint_b: InterfaceAccount<'info, Mint>,
    //Maker should already have an account with token a
    //associated_mint_token represents the token account for a (wallet, token, token_program (classic | 2022))
    //MAker should own the token account they are providing (be the authority)
    //The token account should have been made with the same program as we're using
    //The token account should be for the same instance as token_a
    //Balance check?
    #[account(
        associated_token::authority = maker,
        associated_token::token_program = token_program,
        associated_token::mint = token_mint_a
      )
    ]
    maker_token_a_account: InterfaceAccount<'info, TokenAccount>,

    //The final data struct that will be stored on-chain
    //This must be paid for
    #[account(
      init, 
      payer = maker,
      space = Offer::DISCRIMINATOR.len() + Offer::INIT_SPACE,
      seeds = [b"offer"], //We should add more specific things here, like the instruction id?
      bump
    )]
    offer: Account<'info, Offer>,
    //Create an intermediate account that will hold the users balance
    //We should own the account?
    #[account(
      init,
      payer = maker,
      associated_token::mint = token_mint_a,
      associated_token::authority = offer,
      associated_token::token_program = token_program
    )]
    vault_token_a_account: InterfaceAccount<'info, TokenAccount>,
}

pub fn make_offer(
  ctx: Context<MakeOffer>,
  order_id: u64,
  token_a_out_amount: u64,
  min_token_b_in_amount: u64
) -> Result<()>{
  require!(token_a_out_amount > 0, ErrorCode::InvalidAmount);
  require!(min_token_b_in_amount > 0, ErrorCode::InvalidAmount);

  transfer_tokens(&ctx.accounts.maker_token_a_account, &ctx.accounts.vault_token_a_account, &token_a_out_amount, &ctx.accounts.token_program, &ctx.accounts.token_mint_a, &ctx.accounts.maker).map_err(|_e| ErrorCode::InsufficientMakerBalance)?;

  //Store Offer on-chain
  ctx.accounts.offer.set_inner(Offer { 
    id: order_id,
    maker: ctx.accounts.maker.key(), 
    token_out: ctx.accounts.token_mint_a.key(), 
    token_in: ctx.accounts.token_mint_b.key(), 
    amount_out: token_a_out_amount, 
    min_amount_in: min_token_b_in_amount, 
    bump: ctx.bumps.offer });
  Ok(())
}
