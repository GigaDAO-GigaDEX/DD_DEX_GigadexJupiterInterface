use std::str::FromStr;

use jupiter_amm_interface::{
    AccountMap, Amm, KeyedAccount, Quote, QuoteParams, SwapAndAccountMetas, SwapParams, try_get_account_data,
};
use solana_sdk::pubkey::Pubkey;

use crate::state::{Market, OrderTree};

mod state;

pub struct GigadexOBSwap {
    key: Pubkey,
    market: Market,
    bids : OrderTree,
    asks : OrderTree,
}


impl Amm for GigadexOBSwap {
    fn from_keyed_account(keyed_account: &KeyedAccount) -> Result<Self> {
        let market_state: Market = Market::from(&keyed_account.account.data[0..])?;

        let state = GigadexOBSwap {
            key: keyed_account.key,
            market: market_state,
            bids: OrderTree::default(),
            asks: OrderTree::default(),
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
        vec![self.market.base_mint, self.market.quote_mint]
    }

    fn get_accounts_to_update(&self) -> Vec<Pubkey> {
        vec![self.market.asks, self.market.asks]
    }

    fn update(&mut self, account_map: &AccountMap) -> Result<()> {
        let bids_data = account_map.get(&self.market.bids).unwrap();
        let asks_data = account_map.get(&self.market.asks).unwrap();
        self.asks = OrderTree::from(asks_data.data.as_slice());
        self.bids = OrderTree::from(bids_data.data.as_slice());
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> Result<Quote> {}

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> Result<SwapAndAccountMetas> {}

    fn clone_amm(&self) -> Box<dyn Amm + Send + Sync> {
        Box::new(self.clone())
    }

    fn get_accounts_len(&self) -> usize {
        32
    }
}