use anchor_lang::prelude::Pubkey;
use borsh::{BorshDeserialize, BorshSerialize};

#[repr(C)]
#[derive(Debug, Default, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct Market {
    pub mint: Pubkey,
    pub balances: Pubkey,
    pub wsol_vault: Pubkey,
    pub lot_vault: Pubkey,
    pub asks: Pubkey,
    pub bids: Pubkey,
    pub quote_mint: Pubkey,
}

#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct OrderTree {
    pub root_idx: u64,
    pub market_buy: u64,
    pub nodes: [Node; 1_000],

    pub num_orders: u64,
    pub current_signer: Pubkey,

    // match state
    pub remaining_amount: u64,
    pub num_fills: u64,
    pub fills: [FilledOrder; 64],

    pub num_deltas: u64,
    pub node_delta: [NodeDeltaLog; 64],

    pub amount_cancelled: u64,
}

impl Default for OrderTree {
    fn default() -> Self {
        Self {
            root_idx: 0,
            market_buy: 0,
            nodes: [Node::default(); 1_000],
            num_orders: 0,
            current_signer: Default::default(),
            remaining_amount: 0,
            num_fills: 0,
            fills: [FilledOrder::default(); 64],
            num_deltas: 0,
            node_delta: [NodeDeltaLog::default(); 64],
            amount_cancelled: 0,
        }
    }
}


impl OrderTree {
    pub fn calculate_quote(&self, amount_in: u64, buy: bool) -> u64 {
        let mut remaining_amount = amount_in;
        let mut total_received = 0;

        let mut root_idx = self.root_idx as usize;

        while remaining_amount > 0 {
            let best_price = self.get_best(root_idx);
            if best_price == 0 {
                return 0;
            }

            let order_node = self.get(root_idx);
            let order_amount = order_node.amount;
            if buy {
                if order_amount * best_price > remaining_amount {
                    total_received += remaining_amount / best_price;
                    break;
                } else {
                    total_received += order_amount;
                    remaining_amount -= order_amount * best_price;
                    root_idx =
                        order_node.left as usize
                }
            } else {
                if order_amount > remaining_amount {
                    total_received += remaining_amount * best_price;
                    break;
                } else {
                    total_received += order_amount * best_price;
                    remaining_amount -= order_amount;
                    root_idx =
                        order_node.right as usize
                }
            }
        }

        // 1_000_000 is the precision of the market, constant across all markets
        total_received / 1_000_000
    }

    fn get(&self, idx: usize) -> &Node {
        &self.nodes[idx]
    }

    pub fn get_best(&self, root_idx: usize) -> u64 {
        if root_idx == 0 {
            0
        } else if self.market_buy > 0 {
            // get lowest
            if self.nodes[root_idx].left > 0 {
                self.get_best(self.nodes[root_idx].left as usize)
            } else {
                self.nodes[root_idx].price
            }
        } else {
            // get highest
            if self.nodes[root_idx].right > 0 {
                self.get_best(self.nodes[root_idx].right as usize)
            } else {
                self.nodes[root_idx].price
            }
        }
    }
}


#[derive(Default, Clone, Copy, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Node {
    // key
    pub price: u64,

    // order meta
    pub amount: u64,
    pub uid: u64,

    // indexes
    pub left: u64,
    pub right: u64,
    pub next: u64,

    // balance meta
    pub height: u64,
}

#[derive(Default, Clone, Copy, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct FilledOrder {
    pub price: u64,
    pub amount: u64,
    pub uid: u64,
}

#[derive(Default, Clone, Copy, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct NodeDeltaLog {
    pub key: u64,
    pub is_delete: u64,
    pub is_insert: u64,
    pub is_delta: u64,
    pub amount: u64,
    pub uid: u64,
    pub price: u64,
}

#[derive(Debug, Default, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct FeeMod {
    pub base_fee_bp: u64,
    pub collection_fee_bp: u64,
    pub market_maker_fee_bp: u64,
    pub dex_fee_bp: u64,
    pub collection_royalty_address: Pubkey,
}


#[derive(Debug, Default, PartialEq, Clone, BorshSerialize, BorshDeserialize)]
pub struct AdditionalPdaAccount {
    pub price: u64,
    // ! deprecated
    pub mint: Pubkey,
    pub multiplier: u64,
    pub quote_mint: Pubkey,
    pub fee_receiver_wallet: Pubkey,
}


