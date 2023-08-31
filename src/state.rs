use solana_sdk::pubkey::Pubkey;


#[repr(C)]
#[derive(Debug, Default, PartialEq)]
pub struct Market {
    pub mint: Pubkey,
    pub balances: Pubkey,
    pub wsol_vault: Pubkey,
    pub lot_vault: Pubkey,
    pub asks: Pubkey,
    pub bids: Pubkey,
    pub quote_mint: Pubkey,
}

impl From<&[u8]> for Market {
    fn from(data: &[u8]) -> Self {
        let mint = Pubkey::new(&data[0..32]);
        let balances = Pubkey::new(&data[32..64]);
        let wsol_vault = Pubkey::new(&data[64..96]);
        let lot_vault = Pubkey::new(&data[96..128]);
        let asks = Pubkey::new(&data[128..160]);
        let bids = Pubkey::new(&data[160..192]);
        let quote_mint = Pubkey::new(&data[192..224]);

        Self {
            mint,
            balances,
            wsol_vault,
            lot_vault,
            asks,
            bids,
            quote_mint,
        }
    }
}

#[derive(Default)]
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

pub struct FilledOrder {
    pub price: u64,
    pub amount: u64,
    pub uid: u64,
}

pub struct NodeDeltaLog {
    pub key: u64,
    pub is_delete: u64,
    pub is_insert: u64,
    pub is_delta: u64,
    pub amount: u64,
    pub uid: u64,
    pub price: u64,
}


impl From<&[u8]> for Node {
    fn from(data: &[u8]) -> Self {
        let price = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let amount = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let uid = u64::from_le_bytes(data[16..24].try_into().unwrap());
        let left = u64::from_le_bytes(data[24..32].try_into().unwrap());
        let right = u64::from_le_bytes(data[32..40].try_into().unwrap());
        let next = u64::from_le_bytes(data[40..48].try_into().unwrap());
        let height = u64::from_le_bytes(data[48..56].try_into().unwrap());

        Node {
            price,
            amount,
            uid,
            left,
            right,
            next,
            height,
        }
    }
}

impl From<&[u8]> for FilledOrder {
    fn from(data: &[u8]) -> Self {
        let price = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let amount = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let uid = u64::from_le_bytes(data[16..24].try_into().unwrap());

        FilledOrder { price, amount, uid }
    }
}

impl From<&[u8]> for NodeDeltaLog {
    fn from(data: &[u8]) -> Self {
        let key = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let is_delete = u64::from_le_bytes(data[8..16].try_into().unwrap());
        let is_insert = u64::from_le_bytes(data[16..24].try_into().unwrap());
        let is_delta = u64::from_le_bytes(data[24..32].try_into().unwrap());
        let amount = u64::from_le_bytes(data[32..40].try_into().unwrap());
        let uid = u64::from_le_bytes(data[40..48].try_into().unwrap());
        let price = u64::from_le_bytes(data[48..56].try_into().unwrap());

        NodeDeltaLog {
            key,
            is_delete,
            is_insert,
            is_delta,
            amount,
            uid,
            price,
        }
    }
}

impl From<&[u8]> for OrderTree {
    fn from(data: &[u8]) -> Self {
        let root_idx = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let market_buy = u64::from_le_bytes(data[8..16].try_into().unwrap());

        let mut nodes = [Node::default(); 1_000];
        for i in 0..1_000 {
            let start = 16 + i * 56;
            let end = start + 56;
            nodes[i] = Node::from(&data[start..end]);
        }

        let num_orders = u64::from_le_bytes(data[56016..56024].try_into().unwrap());
        let current_signer = Pubkey::new(&data[56024..56056]);

        let remaining_amount = u64::from_le_bytes(data[56056..56064].try_into().unwrap());
        let num_fills = u64::from_le_bytes(data[56064..56072].try_into().unwrap());

        let mut fills = [FilledOrder::default(); 64];
        for i in 0..64 {
            let start = 56072 + i * 24;
            let end = start + 24;
            fills[i] = FilledOrder::from(&data[start..end]);
        }

        let num_deltas = u64::from_le_bytes(data[71688..71696].try_into().unwrap());

        let mut node_delta = [NodeDeltaLog::default(); 64];
        for i in 0..64 {
            let start = 71696 + i * 56;
            let end = start + 56;
            node_delta[i] = NodeDeltaLog::from(&data[start..end]);
        }

        let amount_cancelled = u64::from_le_bytes(data[87344..87352].try_into().unwrap());

        Self {
            root_idx,
            market_buy,
            nodes,
            num_orders,
            current_signer,
            remaining_amount,
            num_fills,
            fills,
            num_deltas,
            node_delta,
            amount_cancelled,
        }
    }
}
