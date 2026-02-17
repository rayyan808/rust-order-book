//This will be the main entry point for clients interfacing with us

use anchor_lang::prelude::*;

pub mod error;
pub mod handlers;
pub use handlers::*;
pub mod state;
declare_id!("8jR5GeNzeweq35Uo84kGP3v1NcBaZWH5u62k7PxN4T2y");

#[program]
pub mod x {
    use super::*;
    //All of the functions must take in the Context object, wrapped around the
    //input struct (handler) for that function.
    pub fn handle_make_offer(
        context: Context<MakeOffer>,
        id: u64,
        token_a_amount: u64,
        min_token_b_amount: u64,
    ) -> Result<()> {
        handlers::make_offer::make_offer(context, id, token_a_amount, min_token_b_amount)
    }
}
