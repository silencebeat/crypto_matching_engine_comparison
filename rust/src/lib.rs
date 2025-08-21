use std::collections::{BTreeMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side { Buy, Sell }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrdType { Limit, Market }

#[derive(Clone, Debug)]
pub struct Order {
    pub id: u64,
    pub side: Side,
    pub typ: OrdType,
    pub price: i64, // in ticks
    pub qty: i64,   // in units
    pub ts: u64,    // logical time
}

#[derive(Default)]
pub struct Stats {
    pub filled_orders: u64,
    pub filled_qty: i64,
    pub trades: u64,
}

#[derive(Default)]
pub struct MatchResult {
    pub trades: u64,
    pub filled_orders: u64,
    pub filled_qty: i64,
    pub remaining: i64,
}


#[derive(Default)]
pub struct OrderBook {
    // asks: ascending price, bids: descending price (we invert traversal)
    asks: BTreeMap<i64, VecDeque<Order>>,
    bids: BTreeMap<i64, VecDeque<Order>>,
    pub stats: Stats,
    next_ts: u64,
}

impl OrderBook {

    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            next_ts: 0,
            stats: Stats::default(),
        }
    }

    fn push_level(map: &mut BTreeMap<i64, VecDeque<Order>>, price: i64, o: Order) {
        let q = map.entry(price).or_insert_with(VecDeque::new);
        q.push_back(o);
    }

    fn match_against(
        book: &mut BTreeMap<i64, VecDeque<Order>>,
        is_ask_book: bool,
        incoming: &Order,
    ) -> MatchResult {
        let mut rem = incoming.qty;
        let mut trades = 0;
        let mut filled_orders = 0;
        let mut filled_qty = 0;

        loop {
            let best_opt = if is_ask_book {
                book.iter_mut().next().map(|(p, q)| (*p, q))
            } else {
                book.iter_mut().next_back().map(|(p, q)| (*p, q))
            };
            let Some((level_price, queue)) = best_opt else { break };
            let price_ok = match incoming.typ {
                OrdType::Market => true,
                OrdType::Limit => match incoming.side {
                    Side::Buy => level_price <= incoming.price,
                    Side::Sell => level_price >= incoming.price,
                },
            };
            if !price_ok { break; }

            while rem > 0 {
                if let Some(mut head) = queue.pop_front() {
                    let trade_qty = rem.min(head.qty);
                    rem -= trade_qty;
                    head.qty -= trade_qty;
                    trades += 1;
                    filled_qty += trade_qty;
                    if head.qty > 0 {
                        queue.push_front(head);
                        break;
                    } else {
                        filled_orders += 1;
                    }
                } else {
                    break;
                }
            }
            if queue.is_empty() {
                book.remove(&level_price);
            }
            if rem == 0 { break; }
        }

        MatchResult { trades, filled_orders, filled_qty, remaining: rem }
    }

    pub fn submit(&mut self, mut o: Order) {
        self.next_ts += 1;
        o.ts = self.next_ts;

        let result = match o.side {
            Side::Buy => {
                let res = Self::match_against(&mut self.asks, true, &o);
                if res.remaining > 0 && o.typ == OrdType::Limit {
                    o.qty = res.remaining;
                    Self::push_level(&mut self.bids, o.price, o);
                }
                res
            }
            Side::Sell => {
                let res = Self::match_against(&mut self.bids, false, &o);
                if res.remaining > 0 && o.typ == OrdType::Limit {
                    o.qty = res.remaining;
                    Self::push_level(&mut self.asks, o.price, o);
                }
                res
            }
        };

        // update statistik
        self.stats.trades += result.trades;
        self.stats.filled_orders += result.filled_orders;
        self.stats.filled_qty += result.filled_qty;
    }
}
