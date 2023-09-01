use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use jupiter_amm_interface::{AccountMap, Amm, KeyedAccount, Quote, QuoteParams, Side, Swap, SwapAndAccountMetas, SwapParams, try_get_account_data};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::reward_type::RewardType::Fee;

use crate::state::{AdditionalPdaAccount, FeeMod, Market, OrderTree};
use crate::utils::{get_additional_pda, get_fee_mod, get_market_auth_pda};

mod state;
mod utils;
mod constants;

pub struct GigadexOBSwap {
    key: Pubkey,
    market: Market,
    bids: OrderTree,
    asks: OrderTree,
    fee_mod: FeeMod,
    additional_pda: AdditionalPdaAccount,
}


impl Amm for GigadexOBSwap {
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let market_state: Market = Market::from(&keyed_account.account.data[0..]);

        let state = GigadexOBSwap {
            key: keyed_account.key,
            market: market_state,
            bids: OrderTree::default(),
            asks: OrderTree::default(),
            fee_mod: FeeMod::default(),
            additional_pda: AdditionalPdaAccount::default(),
        };
        Ok(state)
    }

    fn label(&self) -> String {
        "GigaDex".to_string()
    }

    fn program_id(&self) -> Pubkey {
        Pubkey::from_str("833pSHchW8AWggrvx8394HHkH1cMHxdyYcDro8ABYUXC").unwrap()
    }

    fn key(&self) -> Pubkey {
        self.key
    }


    fn get_reserve_mints(&self) -> Vec<Pubkey> {
        vec![self.market.mint,
             self.market.quote_mint, ]
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![self.market.asks,
             self.market.asks,
             get_fee_mod(self.key.clone()),
             get_additional_pda(self.key.clone())]
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        let bids_data = account_map.get(&self.market.bids).unwrap();
        let asks_data = account_map.get(&self.market.asks).unwrap();
        let fee_mod_data = account_map.get(&get_fee_mod(self.key.clone())).unwrap();
        let additional_pda_data = account_map.get(&get_additional_pda(self.key.clone())).unwrap();
        self.asks = OrderTree::from(asks_data.data.as_slice());
        self.bids = OrderTree::from(bids_data.data.as_slice());
        self.fee_mod = FeeMod::from(fee_mod_data.data.as_slice());
        self.additional_pda = AdditionalPdaAccount::from(additional_pda_data.data.as_slice());
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {}

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {
        let buy = if swap_params.source_mint == self.market.quote_mint {
            true
        } else {
            false
        };

        let order_tree = if buy {
            &self.asks
        } else {
            &self.bids
        };

        let accounts_metas = [
            AccountMeta::new(swap_params.user_transfer_authority, true),
            AccountMeta::new(self.key, false),
            AccountMeta::new(self.market.balances, false),
            AccountMeta::new(self.market.asks, false),
            AccountMeta::new(self.market.bids, false),
            AccountMeta::new(self.market.wsol_vault, false),
            AccountMeta::new(self.market.lot_vault, false),
            AccountMeta::new(self.additional_pda.fee_receiver_wallet, false),
            AccountMeta::new(self.fee_mod.collection_royalty_address, false),
            AccountMeta::new(get_fee_mod(self.key.into()), false),
            AccountMeta::new(get_additional_pda(self.key.into()), false),
            AccountMeta::new(swap_params.user_source_token_account, false),
            AccountMeta::new(swap_params.user_destination_token_account, false),
            AccountMeta::new(get_market_auth_pda(self.key.into()), false),
            AccountMeta::new_readonly(Token::id(), false),
            AccountMeta::new_readonly(System::id(), false),
        ].to_vec();

        Ok(SwapAndAccountMetas {
            account_metas: accounts_metas,
            swap: Swap::Openbook { side: Side::Bid },
        })
    }

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn get_accounts_len(&self) -> usize {
        32
    }
}