// programs/launchpad/src/error.rs

use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // Time-related errors
    #[msg("Start time must be in the future")]
    StartTimeInPast,

    #[msg("End time must be in the future")]
    InvalidVaultAuthority,

    #[msg("Invalid vault authority")]
    InvalidVaultMint,

    #[msg("End time must be after start time")]
    InvalidTimeRange,

    #[msg("Frozen period must be 1-36 months")]
    InvalidFrozenPeriod,

    #[msg("Unauthorized")]
    ApplicationAlreadySigned,

    #[msg("ApplicationNotApproved Unauthorized")]
    ApplicationNotApproved,

    // Financial limits
    #[msg("Invalid rate (0-10000 allowed)")]
    InvalidRate,

    #[msg("Hard cap exceeded")]
    CapExceeded,

    // Process flow
    #[msg("Funding period has ended")]
    NotInFundingPeriod,

    #[msg("Release time not reached")]
    ReleaseNotYet,

    #[msg("No funds to release")]
    NoFundsToRelease,

    #[msg("Invalid stable coin type (USDC/USDT only)")]
    InvalidCoinType,

    #[msg("String length exceeds limit")]
    InvalidLength,

    #[msg("Mine not approved")]
    MineNotApproved,

    #[msg("Not approved")]
    NotApproved,

    #[msg("Already approved")]
    AlreadyApproved,

    #[msg("Already signed")]
    AlreadySigned,

    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Arithmetic overflow occurred")]
    ArithmeticOverflow,
}