// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for slots
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-03-15, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 128

// Executed Command:
// target/release/polkadot
// benchmark
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=slots
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for slots.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> slots::WeightInfo for WeightInfo<T> {
	fn force_lease() -> Weight {
		(45_648_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn manage_lease_period_start(c: u32, t: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 19_000
			.saturating_add((17_791_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 19_000
			.saturating_add((33_778_000 as Weight).saturating_mul(t as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(t as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(t as Weight)))
	}
}
