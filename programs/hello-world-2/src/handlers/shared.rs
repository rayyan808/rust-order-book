use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

pub fn transfer_tokens<'info>(
    from: &InterfaceAccount<'info, TokenAccount>,
    to: &InterfaceAccount<'info, TokenAccount>,
    amount: &u64,
    token_program: &Interface<'info, TokenInterface>,
    mint: &InterfaceAccount<'info, Mint>,
    authority: &AccountInfo<'info>,
) -> Result<()> {
    let involved_accounts = TransferChecked {
        mint: mint.to_account_info(),
        from: from.to_account_info(),
        to: to.to_account_info(),
        authority: authority.to_account_info(),
    };
    //Convert interface to the account where the program is hosted
    let token_program_account = token_program.to_account_info();

    //Create a Context<> for the instruction we're calling
    let cpi_context = CpiContext::new(token_program_account, involved_accounts);

    //Call Token Program instruction
    //_checked -> We're passing the signer along similar to delegatecall
    //? -> Returns Error if failed
    transfer_checked(cpi_context, *amount, 1)?;
    Ok(())
}
