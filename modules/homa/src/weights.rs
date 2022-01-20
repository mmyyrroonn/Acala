// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_homa
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-12-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=module-homa
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./modules/homa/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for module_homa.
pub trait WeightInfo {
	fn on_initialize() -> Weight;
	fn on_initialize_with_bump_era() -> Weight;
	fn mint() -> Weight;
	fn request_redeem() -> Weight;
	fn fast_match_redeems(n: u32,) -> Weight;
	fn claim_redemption() -> Weight;
	fn update_homa_params() -> Weight;
	fn update_bump_era_params() -> Weight;
	fn reset_ledgers(n: u32,) -> Weight;
	fn reset_current_era() -> Weight;
}

/// Weights for module_homa using the Acala node and recommended hardware.
pub struct AcalaWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for AcalaWeight<T> {
	fn on_initialize() -> Weight {
		(6_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	fn on_initialize_with_bump_era() -> Weight {
		(422_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(29 as Weight))
			.saturating_add(T::DbWeight::get().writes(15 as Weight))
	}
	fn mint() -> Weight {
		(137_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn request_redeem() -> Weight {
		(77_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn fast_match_redeems(n: u32, ) -> Weight {
		(7_163_000 as Weight)
			// Standard Error: 260_000
			.saturating_add((101_229_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	fn claim_redemption() -> Weight {
		(111_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	fn update_homa_params() -> Weight {
		(66_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn update_bump_era_params() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn reset_ledgers(n: u32, ) -> Weight {
		(2_268_000 as Weight)
			// Standard Error: 245_000
			.saturating_add((19_990_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	fn reset_current_era() -> Weight {
		(22_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize() -> Weight {
		(6_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	fn on_initialize_with_bump_era() -> Weight {
		(422_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(29 as Weight))
			.saturating_add(RocksDbWeight::get().writes(15 as Weight))
	}
	fn mint() -> Weight {
		(137_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn request_redeem() -> Weight {
		(77_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn fast_match_redeems(n: u32, ) -> Weight {
		(7_163_000 as Weight)
			// Standard Error: 260_000
			.saturating_add((101_229_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	fn claim_redemption() -> Weight {
		(111_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(7 as Weight))
	}
	fn update_homa_params() -> Weight {
		(66_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	fn update_bump_era_params() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn reset_ledgers(n: u32, ) -> Weight {
		(2_268_000 as Weight)
			// Standard Error: 245_000
			.saturating_add((19_990_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
	}
	fn reset_current_era() -> Weight {
		(22_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
