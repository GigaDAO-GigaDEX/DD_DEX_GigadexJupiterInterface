use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use jupiter_amm_interface::{AccountMap, Amm, KeyedAccount, Quote, QuoteParams, Side, Swap, SwapAndAccountMetas, SwapParams, try_get_account_data};
use solana_sdk::pubkey::Pubkey;

use crate::state::{AdditionalPdaAccount, FeeMod, Market, OrderTree};
use crate::utils::{get_additional_pda, get_fee_mod, get_market_auth_pda};

mod state;
mod utils;
mod constants;


#[derive(Clone)]
pub struct GigadexOBSwap {
    key: Pubkey,
    market: Market,
    bids: OrderTree,
    asks: OrderTree,
    fee_mod: FeeMod,
    additional_pda: AdditionalPdaAccount,
}


impl Amm for GigadexOBSwap {
    fn from_keyed_account(keyed_account: &KeyedAccount) -> std::result::Result<GigadexOBSwap, anyhow::Error> {
        let market_state: Market = Market::deserialize(&mut &keyed_account.account.data[8..])?;


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
             self.market.bids,
             get_fee_mod(self.key.clone()),
             get_additional_pda(self.key.clone())]
    }

    fn update(&mut self, account_map: &AccountMap) -> std::result::Result<(), anyhow::Error> {
        let bids_data = try_get_account_data(account_map, &self.market.bids).unwrap();
        let asks_data = try_get_account_data(account_map, &self.market.asks).unwrap();
        let fee_mod_data = try_get_account_data(account_map, &get_fee_mod(self.key.clone())).unwrap();
        let additional_pda_data = try_get_account_data(account_map, &get_additional_pda(self.key.clone())).unwrap();
        self.asks = OrderTree::deserialize(&mut &asks_data[8..]).unwrap();
        self.bids = OrderTree::deserialize(&mut &bids_data[8..]).unwrap();

        self.fee_mod = FeeMod::deserialize(&mut &fee_mod_data[8..])?;
        self.additional_pda = AdditionalPdaAccount::deserialize(&mut &additional_pda_data[8..])?;
        Ok(())
    }

    fn quote(&self, quote_params: &QuoteParams) -> std::result::Result<Quote, anyhow::Error> {
        let buy = if quote_params.input_mint == self.market.quote_mint {
            true
        } else {
            false
        };

        let order_tree = if buy {
            &self.asks
        } else {
            &self.bids
        };

        let mut amount_out = order_tree.calculate_quote(quote_params.in_amount, buy);

        let fees = (amount_out as f64 * (self.fee_mod.base_fee_bp as f64 / 1e4)) as u64;

        amount_out = amount_out - fees;
        let not_enough_liquidity = amount_out == 0;
        let quote = Quote {
            not_enough_liquidity,
            min_in_amount: Some(quote_params.in_amount),
            min_out_amount: Some(amount_out),
            in_amount: quote_params.in_amount,
            out_amount: amount_out,
            fee_amount: fees,
            fee_mint: self.market.quote_mint,
            fee_pct: Default::default(),
        };
        Ok(quote)
    }

    fn get_swap_and_account_metas(&self, swap_params: &SwapParams) -> std::result::Result<SwapAndAccountMetas, anyhow::Error> {
        let order_tree = if swap_params.source_mint == self.market.quote_mint {
            &self.market.asks
        } else {
            &self.market.bids
        };

        let accounts_metas = [
            AccountMeta::new(swap_params.user_transfer_authority, true),
            AccountMeta::new(self.key, false),
            AccountMeta::new(self.market.balances, false),
            AccountMeta::new(order_tree.to_owned(), false),
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


#[cfg(test)]
mod test {
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::pubkey;

    use super::*;

    #[test]
    // TODO replace std::env by mainnet market after audit deploy
    fn test_jupiter_local() -> Result<()> {
        let market = pubkey!("b457V3EKEy6EB714Y3X4m3xHkDzbTC14iVhNfQZR6rt");

        let rpc = RpcClient::new("https://rpc.hellomoon.io/28632bc6-af26-49e1-93ea-e83334aa3cb0");
        let account = rpc.get_account(&market).unwrap();

        let market_account = KeyedAccount {
            key: market,
            account,
            params: None,
        };

        let mut openbook = GigadexOBSwap::from_keyed_account(&market_account).unwrap();

        let pubkeys = openbook.get_accounts_to_update();

        let accounts: AccountMap = pubkeys
            .iter()
            .zip(rpc.get_multiple_accounts(&pubkeys).unwrap())
            .map(|(key, acc)| (
                *key, acc.unwrap()))
            .collect();

        openbook.update(&accounts).unwrap();

        let (base_mint, quote_mint) = {
            let reserves = openbook.get_reserve_mints();
            (reserves[0], reserves[1])
        };

        let quote_params = QuoteParams {
            in_amount: 10000,
            input_mint: base_mint,
            output_mint: quote_mint,
        };

        let quote = openbook.quote(&quote_params).unwrap();

        println!("{:#?}", quote);

        Ok(())
    }
}