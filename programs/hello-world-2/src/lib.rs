//This will be the main entry point for clients interfacing with us

use anchor_lang::prelude::*;

pub mod handlers;
pub use handlers::*;
pub mod error;
pub mod state;
declare_id!("8jR5GeNzeweq35Uo84kGP3v1NcBaZWH5u62k7PxN4T2y");

#[program]
pub mod main_contract {
    //All of the functions must take in the Context object, wrapped around the
    //input struct (handler) for that function.
    pub fn make_offer(ctx: Context<MakeOffer>) {
        handlers::make_offer()
    }
}
