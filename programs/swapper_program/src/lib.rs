use anchor_lang::prelude::*;

pub mod context;
pub mod dex;
pub mod errors;
pub mod instructions;
pub mod utils;

use anchor_spl::token::TokenAccount;
use context::*;
use errors::SwapError;
use instructions::meteora_swap::handle_dlmm_swap;
use instructions::raydium_swap::swap_base_in;
use utils::evaluate_swap_outcome;

declare_program!(dlmm);
declare_id!("HoM7zCZbeC5gAuu8p86t9GCjFocGqgfySD8xu43vMW1J");

#[program]
pub mod swapper_program {
    use super::*;

    pub fn execute_swap<'info>(
        ctx: Context<'_, '_, '_, 'info, UnifiedSwap<'info>>,
        amount_in: u64,
        min_amount_out: u64,
        quote_mint: Pubkey,
        token_mint: Pubkey,
    ) -> Result<(bool, u64, u64)> {
        // Attempt Raydium swap if the token pair and amount are supported
        if dex::raydium::try_raydium_swap_check(&ctx, token_mint, quote_mint, min_amount_out)? {
            // Read user's source token balance before swap
            let balance_source_before = {
                let mut data_slice_before = &ctx.accounts.user_token_source.data.borrow()[..];
                let source_account_before: TokenAccount =
                    TokenAccount::try_deserialize(&mut data_slice_before)?;
                source_account_before.amount
            };

            // Read user's destination token balance before swap
            let balance_dest_before = {
                let mut data_slice_before = &ctx.accounts.user_token_destination.data.borrow()[..];
                let destination_account_before: TokenAccount =
                    TokenAccount::try_deserialize(&mut data_slice_before)?;
                destination_account_before.amount
            };

            // Perform the Raydium swap via CPI call
            let (_destination_account, balance_dest_after, _source_account, balance_source_after) =
                swap_base_in(ctx, amount_in, min_amount_out)?;

            // Evaluate swap results (amount received, fees, slippage checks)
            return evaluate_swap_outcome(
                balance_source_before,
                balance_source_after,
                balance_dest_before,
                balance_dest_after,
                amount_in,
                min_amount_out,
            )
            // Convert custom SwapError into Anchor's error type for consistent error handling
            .map_err(Into::into);
        } else {
            // If Raydium swap not possible, attempt Meteora swap
            if dex::meteora::try_meteora_swap_check(&ctx, token_mint, quote_mint, min_amount_out)? {
                // Read user's source token balance before Meteora swap
                let balance_source_before = {
                    let mut data_slice_before = &ctx.accounts.user_token_in.data.borrow()[..];
                    let source_account_before: TokenAccount =
                        TokenAccount::try_deserialize(&mut data_slice_before)?;
                    source_account_before.amount
                };

                // Read user's destination token balance before Meteora swap
                let balance_dest_before = {
                    let mut data_slice_before = &ctx.accounts.user_token_out.data.borrow()[..];
                    let destination_account_before: TokenAccount =
                        TokenAccount::try_deserialize(&mut data_slice_before)?;
                    destination_account_before.amount
                };

                // Perform the Meteora swap via CPI call
                let (
                    _destination_account,
                    balance_dest_after,
                    _source_account,
                    balance_source_after,
                ) = handle_dlmm_swap(ctx, amount_in, min_amount_out)?;

                // Evaluate swap results (amount received, fees, slippage checks)
                return evaluate_swap_outcome(
                    balance_source_before,
                    balance_source_after,
                    balance_dest_before,
                    balance_dest_after,
                    amount_in,
                    min_amount_out,
                )
                .map_err(Into::into);
            }
        }

        // ----
        // If we reach here, neither Raydium nor Meteora supported the token pair and amounts.
        // This is where you can add support for other DEXes by adding similar blocks like above:
        //
        // if dex::other_dex::try_other_dex_swap_check(...) {
        //     // read balances before
        //     // perform swap via CPI
        //     // read balances after
        //     // evaluate and return result
        // }
        //
        // ----

        // Return error if no supported swap route was found for the token pair and amounts
        Err(SwapError::UnsupportedTokenPairr.into())
    }
}
