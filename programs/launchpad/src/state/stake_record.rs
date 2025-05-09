use anchor_lang::prelude::*;

#[account]
pub struct StakeRecord {
    // Participant information
    pub investor:   Pubkey,  // Investor wallet address

    // Investment details
    pub mine_code:  String,  // Associated mine identifier
    pub stable_coin:String,  // Payment currency ("USDC" or "USDT")
    pub amount:     u128,    // Investment amount

    // Transaction metadata
    pub txn_hash:   String,  // On-chain transaction hash
    pub timestamp:  i64,     // Investment timestamp

    // Technical fields
    pub bump:       u8,      // PDA bump
}

impl StakeRecord {
    // Constants for string storage
    pub const MAX_MINE_CODE_LEN: usize = 32;
    pub const MAX_COIN_LEN: usize = 8;
    pub const MAX_TXN_HASH_LEN: usize = 128;

    // Account storage size calculation
    pub const LEN: usize =
        32 +  // investor: Pubkey

            // String fields (prefix + content)
            (4 + Self::MAX_MINE_CODE_LEN) +  // mine_code
            (4 + Self::MAX_COIN_LEN) +       // stable_coin
            (4 + Self::MAX_TXN_HASH_LEN) +   // txn_hash

            // Numeric fields
            16 +  // amount: u128
            8 +  // timestamp: i64

            // Technical field
            1;   // bump: u8
}