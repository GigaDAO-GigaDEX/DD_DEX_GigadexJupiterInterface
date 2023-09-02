use anchor_lang::prelude::*;
use solana_sdk::pubkey;


pub const GIGADEX_PROGRAM_ID : Pubkey = pubkey!("833pSHchW8AWggrvx8394HHkH1cMHxdyYcDro8ABYUXC");

pub const MARKET_AUTH_PDA_SEED: &[u8] = b"market_auth_pda_seed";
pub const FEE_MOD_PDA_SEED: &[u8] = b"fee_mod_pda_seed";
pub const ADDITIONAL_PDA_SEED: &[u8] = b"additional_pda_seed";