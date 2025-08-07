use crate::errors::SwapError;
use crate::UnifiedSwap;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount as SplTokenAccount;

pub fn try_meteora_swap_check(
    ctx: &Context<UnifiedSwap>,
    token_mint: Pubkey,
    quote_mint: Pubkey,
    min_amount_out: u64,
) -> Result<bool> {
    let accounts = &ctx.accounts;

    let token_x_match = accounts.token_x_mint.key() == token_mint;
    let token_y_match = accounts.token_y_mint.key() == quote_mint;

    if !(token_x_match && token_y_match) {
        return Ok(false);
    }

    let reserve_y_info = accounts.reserve_y.to_account_info(); // keep alive
    let reserve_y_data = reserve_y_info.try_borrow_data()?;
    let reserve_y_token = SplTokenAccount::try_deserialize(&mut &reserve_y_data[..])?;

    if reserve_y_token.amount < min_amount_out {
        return Err(SwapError::InsufficientLiquidity.into());
    }

    Ok(true)
}
