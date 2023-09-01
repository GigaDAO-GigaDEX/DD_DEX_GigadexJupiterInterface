use anchor_lang::prelude::*;

use crate::constants::{GIGADEX_PROGRAM_ID, MARKET_AUTH_PDA_SEED, ADDITIONAL_PDA_SEED, FEE_MOD_PDA_SEED};

pub fn get_market_auth_pda(market_address: Pubkey) -> Pubkey {
    let (market_auth_pda, _) = Pubkey::find_program_address(
        &[market_address.as_ref(), MARKET_AUTH_PDA_SEED.as_ref()],
        &GIGADEX_PROGRAM_ID,
    );
    market_auth_pda
}

pub fn get_additional_pda(market_address: Pubkey) -> Pubkey {
    let (additional_pda, _) = Pubkey::find_program_address(
        &[market_address.as_ref(), ADDITIONAL_PDA_SEED.as_ref()],
        &GIGADEX_PROGRAM_ID,
    );
    additional_pda
}

pub fn get_fee_mod(market_address: Pubkey) -> Pubkey {
    let (fee_mod, _) = Pubkey::find_program_address(
        &[market_address.as_ref(), FEE_MOD_PDA_SEED.as_ref()],
        &GIGADEX_PROGRAM_ID,
    );
    fee_mod
}