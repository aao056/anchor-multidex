use anchor_lang::prelude::*;

#[error_code]
pub enum SwapError {
    #[msg("Math error occured.")]
    MathError,

    #[msg("Meteora swap failed.")]
    MeteoraSwapFailed,

    #[msg("Raydium swap failed.")]
    RaydiumSwapFailed,

    #[msg("Insufficient liquidity in selected DEX.")]
    InsufficientLiquidity,

    #[msg("Minimum output not met due to slippage.")]
    ExcessiveSlippage,

    #[msg("Unsupported token pair.")]
    UnsupportedTokenPairr,
}
