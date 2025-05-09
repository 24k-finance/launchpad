use anchor_lang::prelude::*;

#[account]
pub struct LaunchPool {
    // Pool configuration
    pub authority:    Pubkey,    // Creator/admin address
    pub price:        u64,       // Token price (in payment token units)
    pub cap:          u64,       // Hard cap for fundraising
    pub raised:       u64,       // Amount already raised

    // Timing parameters
    pub start_time:   i64,       // Fundraising start timestamp
    pub end_time:     i64,       // Fundraising end timestamp

    // Business parameters
    pub mine_code:    String,    // Associated mine identifier
    pub rate:         u32,       // Annualized rate (actual value divided by 10000)
    pub frozen_month: u8,        // Token lock-up period in months

    // Technical fields
    pub bump:         u8,        // PDA bump

    // Token vaults
    pub usdc_mint:    Pubkey,    // USDC mint address
    pub usdc_vault:   Pubkey,    // USDC vault PDA
    pub usdt_mint:    Pubkey,    // USDT mint address
    pub usdt_vault:   Pubkey,    // USDT vault PDA
}

impl LaunchPool {
    // Constants for string storage
    pub const MAX_MINE_CODE_LEN: usize = 32;

    // Account storage size calculation
    pub const LEN: usize =
    // Configuration fields (authority + numeric parameters)
        32 + (8 * 5) +

            // Mine code string (prefix + content)
            4 + Self::MAX_MINE_CODE_LEN +

            // Business parameters
            4 + 1 + 1 +  // rate + frozen_month + bump

            // Token vault addresses
            32 * 4;
}