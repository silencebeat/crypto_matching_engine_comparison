from collections import defaultdict, deque
import heapq

BUY = 1
SELL = 2
LIMIT = 1
MARKET = 2

class Order:
    __slots__ = ("id","side","typ","price","qty","ts")
    def __init__(self, id:int, side:int, typ:int, price:int, qty:int, ts:int=0):
        self.id=id; self.side=side; self.typ=typ; self.price=price; self.qty=qty; self.ts=ts

class OrderBook:
    def __init__(self):
        # heaps of price levels, asks min-heap, bids max-heap (store -price)
        self.asks = []      # (price, key)
        self.bids = []      # (-price, key)
        self.ask_map = {}   # price -> deque
        self.bid_map = {}   # price -> deque
        self.ts = 0
        self.trades=0; self.filled_orders=0; self.filled_qty=0

    def _push_level(self, side_map, heap, is_ask, price, o):
        dq = side_map.get(price)
        if dq is None:
            dq = deque()
            side_map[price] = dq
            heapq.heappush(heap, (price if is_ask else -price, price))
        dq.append(o)

    def _match(self, side_map, heap, is_ask_book, incoming: Order):
        rem = incoming.qty
        while rem>0 and heap:
            _, best_price = heap[0]
            dq = side_map.get(best_price)
            if not dq:
                heapq.heappop(heap)
                continue
            # price check
            if incoming.typ == MARKET:
                price_ok = True
            else:
                price_ok = (incoming.side==BUY and best_price<=incoming.price) or (incoming.side==SELL and best_price>=incoming.price)
            if not price_ok:
                break

            while rem>0 and dq:
                head = dq[0]
                trade = min(rem, head.qty)
                rem -= trade; head.qty -= trade
                self.trades += 1; self.filled_qty += trade
                if head.qty==0:
                    dq.popleft(); self.filled_orders+=1
                else:
                    break
            if not dq:
                side_map.pop(best_price, None)
                heapq.heappop(heap)
        return rem

    def submit(self, o: Order):
        self.ts += 1; o.ts = self.ts
        if o.side == BUY:
            rem = self._match(self.ask_map, self.asks, True, o)
            if rem>0 and o.typ==LIMIT:
                o.qty = rem
                self._push_level(self.bid_map, self.bids, False, o.price, o)
        else:
            rem = self._match(self.bid_map, self.bids, False, o)
            if rem>0 and o.typ==LIMIT:
                o.qty = rem
                self._push_level(self.ask_map, self.asks, True, o.price, o)
