use crate::errors::SwapError;

pub fn evaluate_swap_outcome(
    balance_source_before: u64,
    balance_source_after: u64,
    balance_dest_before: u64,
    balance_dest_after: u64,
    amount_in: u64,
    min_amount_out: u64,
) -> Result<(bool, u64, u64), SwapError> {
    let amount_received = balance_dest_after
        .checked_sub(balance_dest_before)
        .ok_or(SwapError::MathError)?;

    let actual_amount_spent = balance_source_before
        .checked_sub(balance_source_after)
        .ok_or(SwapError::MathError)?;

    let fee_paid = amount_in.checked_sub(actual_amount_spent).unwrap_or(0);

    if amount_received < min_amount_out {
        return Err(SwapError::ExcessiveSlippage.into());
    }

    Ok((true, amount_received, fee_paid))
}
