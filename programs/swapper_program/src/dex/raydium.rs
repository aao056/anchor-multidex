use crate::errors::SwapError;
use crate::UnifiedSwap;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount as SplTokenAccount;

pub fn try_raydium_swap_check(
    ctx: &Context<UnifiedSwap>,
    token_mint: Pubkey,
    quote_mint: Pubkey,
    min_amount_out: u64,
) -> Result<bool> {
    let accounts = &ctx.accounts;

    let input_mint_match = accounts.amm_pc_vault.key() == token_mint;
    let output_mint_match = accounts.amm_coin_vault.key() == quote_mint;

    if !(input_mint_match && output_mint_match) {
        return Ok(false);
    }

    let output_vault_info = accounts.amm_coin_vault.to_account_info();
    let output_vault_data = output_vault_info.try_borrow_data()?;
    let output_vault_token = SplTokenAccount::try_deserialize(&mut &output_vault_data[..])?;

    if output_vault_token.amount < min_amount_out {
        return Err(SwapError::InsufficientLiquidity.into());
    }

    Ok(true)
}
