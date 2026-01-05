// Dynamically linking wasm modules is painful and difficult. For this reason
// we reexport read-fonts, write-fonts, and Skrifa symbols here to create a
// mega-module.
// use std::ops::RangeInclusive;
// use skrifa::{FontRef, MetadataProvider};
// use wasm_bindgen::prelude::*;
// use write_fonts::read::collections::IntSet;
//
// #[wasm_bindgen]
// pub struct IntSet32(IntSet<u32>);
//
// #[derive(tsify::Tsify, serde::Serialize)]
// #[tsify(into_wasm_abi)]
// pub struct Range([u32; 2]);
//
// #[wasm_bindgen]
// impl IntSet32 {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> IntSet32 {
//         IntSet32(IntSet::<u32>::new())
//     }
//     pub fn insert(&mut self, a: u32) {
//         self.0.insert(a);
//     }
//
//     pub fn remove(&mut self, a: u32) {
//         self.0.remove(a);
//     }
//
//     pub fn remove_range(&mut self, from: u32, to: u32) {
//         self.0.remove_range(RangeInclusive::<u32>::new(from, to));
//     }
//
//     pub fn insert_range(&mut self, from: u32, to: u32) {
//         self.0.insert_range(RangeInclusive::<u32>::new(from, to));
//     }
//
//     pub fn invert(&mut self) {
//         self.0.invert();
//     }
//
//     #[wasm_bindgen(getter)]
//     pub fn length(&mut self) -> u64 {
//         self.0.len()
//     }
//
//     pub fn iter_ranges(&mut self) -> Vec<Range> {
//         self.0
//             .iter_ranges()
//             .map(|a| Range([a.start().clone(), a.end().clone()]))
//             .collect()
//     }
//
//     pub fn intersect(&mut self, other: &IntSet32) {
//         self.0.intersect(&other.0);
//     }
//
//     pub fn subtract(&mut self, other: &IntSet32) {
//         self.0.subtract(&other.0);
//     }
// }
//
// impl FromIterator<u32> for IntSet32 {
//     fn from_iter<I: IntoIterator<Item = u32>>(iter: I) -> Self {
//         let mut s = IntSet32::new();
//         s.0.extend(iter);
//         s
//     }
// }

// #[derive(tsify::Tsify, serde::Serialize)]
// #[tsify(into_wasm_abi)]
// #[wasm_bindgen]
// pub struct Font(FontRef<'static>);
//
// #[wasm_bindgen]
// impl Font {
// 		#[wasm_bindgen(constructor)]
// 		pub fn new(data: &[u8]) -> Result<Font, skrifa::raw::ReadError> {
// 			Ok(Font(skrifa::FontRef::new(&data)?))
// 		}
//
// 	pub fn unicodes(self: Self) -> IntSet32 {
// 		self.0.charmap().mappings().map(|t| t.0).collect::<IntSet32>()
// 	}
// }
