package main

import (
	"fmt"
	"math/rand"
	"os"
	"strconv"
	"time"

	"matchingengine/engine"
)

func main() {
	n := 3000000
	if len(os.Args) > 1 {
		if v, err := strconv.Atoi(os.Args[1]); err == nil {
			n = v
		}
	}
	marketPct := 10
	if len(os.Args) > 2 {
		if v, err := strconv.Atoi(os.Args[2]); err == nil {
			marketPct = v
		}
	}

	ob := engine.NewOrderBook()
	rng := rand.New(rand.NewSource(42))

	t0 := time.Now()
	for id := 0; id < n; id++ {
		side := engine.Buy
		if rng.Float64() < 0.5 {
			side = engine.Sell
		}
		isMarket := rng.Intn(100) < marketPct
		typ := engine.Limit
		if isMarket { typ = engine.Market }
		priceBase := int64(100_000)
		drift := rng.Int63n(10_001) - 5_000
		price := priceBase + drift
		qty := int64(rng.Intn(3)+1)
		o := engine.Order{
			Id: uint64(id), Side: side, Type: typ, Price: price, Qty: qty,
		}
		ob.Submit(o)
	}
	elapsed := time.Since(t0)
	fmt.Printf("Go: processed %d orders in %v\n", n, elapsed)
	fmt.Printf("trades=%d filled_orders=%d filled_qty=%d\n", ob.Trades, ob.FilledOrders, ob.FilledQty)
	fmt.Printf("throughput ~ %.0f orders/sec\n", float64(n)/elapsed.Seconds())
}
