use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Amount supplied was invalid")]
    InvalidAmount,
}
