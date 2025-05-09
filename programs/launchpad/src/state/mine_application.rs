use anchor_lang::prelude::*;

#[account]
pub struct MineApplication {
    // Basic information
    pub owner: Pubkey,     // Owner wallet address
    pub mine_code: String, // Platform-generated mine ID
    pub name: String,      // Official mine name
    pub operator: String,  // Operator/management company name

    // Documentation and verification
    pub relationship: String, // URI to relationship proof document
    pub approval1: String,    // URI to government authorization document
    pub approval2: String,    // URI to construction permit document
    pub approval3: String,    // URI to other policy documents

    // Mine characteristics
    pub scale: String,    // Mine scale/classification
    pub location: String, // Geographic location

    // Financing terms
    pub finance_scale: u128, // Financing target amount (in currency units)
    pub currency: String,    // Currency type (e.g., "CNY", "USD")
    pub start_date: i64,     // Financing start timestamp
    pub end_date: i64,       // Financing end timestamp
    pub rate: u32,           // Annualized rate (actual value divided by 10000)
    pub frozen_month: u8,    // Lock-up period in months

    // Status flags
    pub audit_result: bool,     // Approval status
    pub is_signed: bool,        // Whether contract is signed on-chain
    pub sign_date: Option<i64>, // Timestamp of signing

    // Technical fields
    pub bump: u8, // PDA bump
}

impl MineApplication {
    // String field maximum lengths
    pub const MAX_MINE_CODE_LEN: usize = 32;
    pub const MAX_NAME_LEN: usize = 64;
    pub const MAX_OPERATOR_LEN: usize = 64;
    pub const MAX_RELATIONSHIP_LEN: usize = 128;
    pub const MAX_SCALE_LEN: usize = 32;
    pub const MAX_LOCATION_LEN: usize = 64;
    pub const MAX_APPROVAL_LEN: usize = 128;
    pub const MAX_CURRENCY_LEN: usize = 8;

    // Account storage size calculation
    pub const LEN: usize = 32
        + (4 + Self::MAX_MINE_CODE_LEN)
        + (4 + Self::MAX_NAME_LEN)
        + (4 + Self::MAX_OPERATOR_LEN)
        + (4 + Self::MAX_RELATIONSHIP_LEN)
        + (4 + Self::MAX_SCALE_LEN)
        + (4 + Self::MAX_LOCATION_LEN)
        + (4 + Self::MAX_APPROVAL_LEN) * 3
        + 16
        + (4 + Self::MAX_CURRENCY_LEN)
        + 8
        + 8
        + 4
        + 1
        + 1
        + 1
        + (1 + 8)
        + 1;
}
