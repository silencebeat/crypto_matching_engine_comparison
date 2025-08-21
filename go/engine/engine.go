package engine

import (
	"slices"
)

type Side int
const (
	Buy Side = iota
	Sell
)

type OrdType int
const (
	Limit OrdType = iota
	Market
)

type Order struct {
	Id    uint64
	Side  Side
	Type  OrdType
	Price int64
	Qty   int64
	Ts    uint64
}

type level struct {
	price int64
	q     []Order // FIFO
}

type OrderBook struct {
	asks []level // ascending price
	bids []level // descending price
	nextTs uint64

	Trades uint64
	FilledOrders uint64
	FilledQty int64
}

func NewOrderBook() *OrderBook {
	return &OrderBook{}
}

func (ob *OrderBook) pushLevel(levels *[]level, price int64, o Order, asc bool) {
	// find price level
	idx := slices.IndexFunc(*levels, func(l level) bool { return l.price == price })
	if idx >= 0 {
		(*levels)[idx].q = append((*levels)[idx].q, o)
		return
	}
	// insert keeping order
	l := level{price: price, q: []Order{o}}
	*levels = append(*levels, l)
	if asc {
		slices.SortFunc(*levels, func(a, b level) int {
			if a.price < b.price { return -1 }
			if a.price > b.price { return 1 }
			return 0
		})
	} else {
		slices.SortFunc(*levels, func(a, b level) int {
			if a.price > b.price { return -1 }
			if a.price < b.price { return 1 }
			return 0
		})
	}
}

func (ob *OrderBook) matchAgainst(levels *[]level, isAskBook bool, incoming Order) int64 {
	rem := incoming.Qty
	for rem > 0 && len(*levels) > 0 {
		// best price at index 0 due to sorting
		best := &(*levels)[0]
		priceOk := incoming.Type == Market ||
			(incoming.Side == Buy && best.price <= incoming.Price) ||
			(incoming.Side == Sell && best.price >= incoming.Price)
		if !priceOk { break }

		// consume FIFO
		for rem > 0 && len(best.q) > 0 {
			head := best.q[0]
			trade := head.Qty
			if trade > rem { trade = rem }
			rem -= trade
			head.Qty -= trade
			ob.Trades++
			ob.FilledQty += trade
			if head.Qty > 0 {
				best.q[0] = head
				break
			} else {
				ob.FilledOrders++
				best.q = best.q[1:]
			}
		}
		if len(best.q) == 0 {
			// remove level
			*levels = (*levels)[1:]
		}
	}
	return rem
}

func (ob *OrderBook) Submit(o Order) {
	ob.nextTs++
	o.Ts = ob.nextTs
	if o.Side == Buy {
		rem := ob.matchAgainst(&ob.asks, true, o)
		if rem > 0 && o.Type == Limit {
			o.Qty = rem
			ob.pushLevel(&ob.bids, o.Price, o, false)
		}
	} else {
		rem := ob.matchAgainst(&ob.bids, false, o)
		if rem > 0 && o.Type == Limit {
			o.Qty = rem
			ob.pushLevel(&ob.asks, o.Price, o, true)
		}
	}
}
