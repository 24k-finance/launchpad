use anchor_lang::prelude::*;

#[event]
pub struct PoolInitialized {
    pub mine_code: String,      
    pub pool: Pubkey,           
    pub timestamp: i64,         
}

#[event]
pub struct FundsStaked {
    pub investor: Pubkey,       
    pub pool: Pubkey,           
    pub amount: u64,           
    pub timestamp: i64,
}

#[event]
pub struct FundsReleased {
    pub pool: Pubkey,           
    pub receiver: Pubkey,       
    pub amount: u64,            
    pub timestamp: i64,
}

#[event]
pub struct MineApplied {
    pub mine_code: String,
    pub owner: Pubkey,
    pub timestamp: i64
}

#[event]
pub struct MineApproved {
    pub mine_code: String,
    pub timestamp: i64
}

#[event]
pub struct MineSigned {
    pub mine_code: String,
    pub pool_address: Pubkey, 
    pub timestamp: i64
}