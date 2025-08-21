export enum Side { Buy, Sell }
export enum OrdType { Limit, Market }

export interface Order {
  id: number; side: Side; typ: OrdType; price: number; qty: number; ts: number;
}

type Level = { price:number, q: Order[] };

export class OrderBook {
  asks: Level[] = []; // ascending
  bids: Level[] = []; // descending
  nextTs = 0;
  trades = 0; filledOrders = 0; filledQty = 0;

  private pushLevel(levels: Level[], price:number, o:Order, asc:boolean) {
    const idx = levels.findIndex(l => l.price===price);
    if (idx>=0) { levels[idx].q.push(o); return; }
    levels.push({price, q:[o]});
    levels.sort((a,b)=> asc ? a.price-b.price : b.price-a.price);
  }

  private matchAgainst(levels: Level[], isAskBook:boolean, incoming: Order): number {
    let rem = incoming.qty;
    while (rem>0 && levels.length>0) {
      const best = levels[0];
      const priceOk = incoming.typ===OrdType.Market ||
        (incoming.side===Side.Buy && best.price<=incoming.price) ||
        (incoming.side===Side.Sell && best.price>=incoming.price);
      if (!priceOk) break;
      while (rem>0 && best.q.length>0) {
        const head = best.q[0];
        const trade = Math.min(rem, head.qty);
        rem -= trade; head.qty -= trade;
        this.trades++; this.filledQty += trade;
        if (head.qty===0) { best.q.shift(); this.filledOrders++; } else { break; }
      }
      if (best.q.length===0) { levels.shift(); }
    }
    return rem;
  }

  submit(o: Order) {
    this.nextTs++; o.ts = this.nextTs;
    if (o.side===Side.Buy) {
      const rem = this.matchAgainst(this.asks, true, o);
      if (rem>0 && o.typ===OrdType.Limit) { o.qty=rem; this.pushLevel(this.bids, o.price, o, false); }
    } else {
      const rem = this.matchAgainst(this.bids, false, o);
      if (rem>0 && o.typ===OrdType.Limit) { o.qty=rem; this.pushLevel(this.asks, o.price, o, true); }
    }
  }
}
