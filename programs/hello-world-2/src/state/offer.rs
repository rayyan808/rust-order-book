use anchor_lang::prelude::*;

use anchor_spl::token::Mint;
use anchor_spl::token_interface::TokenInterface;

#[account]
//Offer is stored on-chain, so its an account
//We derive InitSpace so we can easily calculate how much space it'll need on-chain
//and how much fees it cost to rent this space
#[derive(InitSpace)]
pub struct Offer {
    pub maker: Pubkey,
    pub token_out: Pubkey,
    pub token_in: Pubkey,
    pub amount_out: u64,
    pub min_amount_in: u64,
    pub bump: u8, //Store bump so we can calculate the address of 'Offer' quickly
}
