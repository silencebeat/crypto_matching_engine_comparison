import { OrderBook, Side, OrdType } from "./engine.js";

const n = process.argv[2] ? parseInt(process.argv[2],10) : 3_000_000;
const marketPct = process.argv[3] ? parseInt(process.argv[3],10) : 10;

const ob = new OrderBook();
function rand(seed:number){ let x=seed; return ()=> (x = (x*1664525 + 1013904223) >>> 0) / 2**32 }
const r = rand(42);

const t0 = performance.now();
for (let i=0;i<n;i++){
  const side = (r()<0.5) ? Side.Buy : Side.Sell;
  const isMarket = (r()*100)<marketPct;
  const typ = isMarket ? OrdType.Market : OrdType.Limit;
  const price = 100_000 + Math.floor((r()*10001)-5000);
  const qty = 1 + Math.floor(r()*3);
  ob.submit({id:i, side, typ, price, qty, ts:0});
}
const dtMs = performance.now()-t0;
console.log(`TS/Node: processed ${n} orders in ${(dtMs/1000).toFixed(3)}s`);
console.log(`trades=${ob.trades} filled_orders=${ob.filledOrders} filled_qty=${ob.filledQty}`);
console.log(`throughput ~ ${(n/(dtMs/1000)).toFixed(0)} orders/sec`);
