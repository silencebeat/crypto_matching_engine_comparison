use clap::Parser;
use rand::{Rng, rngs::StdRng, SeedableRng};
use matching_engine::{OrderBook, Order, Side, OrdType};
use std::time::Instant;

#[derive(Parser, Debug)]
struct Args {
    /// jumlah order
    n: usize,
    /// persentase market order (0-100)
    #[arg(short, long, default_value_t = 10)]
    market_pct: u32,
}

fn main() {
    let args = Args::parse();
    let n = args.n;
    let mut ob = OrderBook::new();
    let mut rng = StdRng::seed_from_u64(42);

    let t0 = Instant::now();
    for id in 0..n as u64 {
        let side = if rng.gen_bool(0.5) { Side::Buy } else { Side::Sell };
        let is_market = rng.gen_range(0..100) < args.market_pct;
        let typ = if is_market { OrdType::Market } else { OrdType::Limit };
        let price_base: i64 = 100_000;
        let drift = rng.gen_range(-5_000..=5_000);
        let price = price_base + drift;
        let qty = rng.gen_range(1..=3);
        let o = Order { id, side, typ, price, qty, ts: 0 };
        ob.submit(o);
    }
    let dt = t0.elapsed();
    println!("Rust: processed {} orders in {:.3?}", n, dt);
    println!("trades={}, filled_orders={}, filled_qty={}", ob.stats.trades, ob.stats.filled_orders, ob.stats.filled_qty);
    println!("throughput ~ {:.0} orders/sec", n as f64 / dt.as_secs_f64());
}
