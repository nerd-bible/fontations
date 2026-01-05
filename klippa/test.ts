import { IntSet32 } from "./pkg/klippa.js";

const set = new IntSet32();
set.insert_range(0, 50);
set.insert_range(100, 200);
for (const r of set.iter_ranges()) {
	console.log(r);
}
