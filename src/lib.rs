//! This crate provides data collections such as `DisjointVec` and
//! `DisjointHashMap` where items are categorized into disjoint *classes*.
mod vec;
pub use vec::DisjointVec;

mod hash_map;
pub use hash_map::DisjointHashMap;
