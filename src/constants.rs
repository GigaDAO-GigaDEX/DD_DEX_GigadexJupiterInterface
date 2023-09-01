use std::str::FromStr;
use anchor_lang::prelude::*;


pub const GIGADEX_PROGRAM_ID : Pubkey = Pubkey::from_str("833pSHchW8AWggrvx8394HHkH1cMHxdyYcDro8ABYUXC").unwrap();

pub const MARKET_AUTH_PDA_SEED: &[u8] = b"market_auth_pda_seed";
pub const SELL_LOG_PDA_SEED: &[u8] = b"sell_log_pda_seed";
pub const BUY_LOG_PDA_SEED: &[u8] = b"buy_log_pda_seed";
pub const FEE_MOD_PDA_SEED: &[u8] = b"fee_mod_pda_seed";
pub const ADDITIONAL_PDA_SEED: &[u8] = b"additional_pda_seed";