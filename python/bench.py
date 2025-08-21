import sys, time, random
from engine import OrderBook, Order, BUY, SELL, LIMIT, MARKET

def main():
    n = int(sys.argv[1]) if len(sys.argv)>1 else 3_000_000
    market_pct = int(sys.argv[2]) if len(sys.argv)>2 else 10
    ob = OrderBook()
    rng = random.Random(42)
    t0 = time.time()
    for i in range(n):
        side = BUY if rng.random()<0.5 else SELL
        is_market = rng.randint(0,99) < market_pct
        typ = MARKET if is_market else LIMIT
        price = 100_000 + rng.randint(-5000,5000)
        qty = rng.randint(1,3)
        ob.submit(Order(i, side, typ, price, qty))
    dt = time.time()-t0
    print(f"Python: processed {n} orders in {dt:.3f}s")
    print(f"trades={ob.trades} filled_orders={ob.filled_orders} filled_qty={ob.filled_qty}")
    print(f"throughput ~ {n/dt:.0f} orders/sec")

if __name__=="__main__":
    main()
