// Copyright 2019 multiset developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A multiset is an unordered collection of values. They are also
//! known as bags.
//!
//! Unlike sets where each value is either included or not, multisets
//! permit duplicates. Consequently, they're useful for maintaining a
//! count of distinct values.

#[macro_use]
mod multiset;

mod btree_multiset {
	__impl_multiset! {BTreeMultiSet, std::collections::BTreeMap, std::collections::btree_map, Ord}
}
mod hash_multiset {
	use std::hash::Hash;
	__impl_multiset! {HashMultiSet, std::collections::HashMap, std::collections::hash_map, Eq;Hash}
}
mod iter;

pub use btree_multiset::BTreeMultiSet;
pub use hash_multiset::HashMultiSet;
pub use iter::Iter;
