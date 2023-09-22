// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `da_control`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-14, STEPS: `100`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-172-31-12-189`, CPU: `Intel(R) Xeon(R) Platinum 8175M CPU @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/data-avail
// benchmark
// pallet
// --chain=dev
// --steps=100
// --repeat=20
// --pallet=da_control
// --extrinsic=*
// --heap-pages=4096
// --header=./HEADER-APACHE2
// --log=warn
// --output
// ./output/da_control_weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `da_control`.
pub trait WeightInfo {
	fn create_application_key() -> Weight;
	fn submit_block_length_proposal() -> Weight;
	fn submit_data(i: u32, ) -> Weight;
	fn data_root(i: u32, ) -> Weight;
	fn data_root_batch(i: u32, ) -> Weight;
	fn commitment_builder_32(i: u32, ) -> Weight;
	fn commitment_builder_64(i: u32, ) -> Weight;
	fn commitment_builder_128(i: u32, ) -> Weight;
	fn commitment_builder_256(i: u32, ) -> Weight;
}

/// Weights for `da_control` using the Avail node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `DataAvailability::AppKeys` (r:1 w:1)
	/// Proof: `DataAvailability::AppKeys` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DataAvailability::NextAppId` (r:1 w:1)
	/// Proof: `DataAvailability::NextAppId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn create_application_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117`
		//  Estimated: `3583`
		// Minimum execution time: 28_304_000 picoseconds.
		Weight::from_parts(29_864_000, 3583)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `System::DynamicBlockLength` (r:1 w:1)
	/// Proof: `System::DynamicBlockLength` (`max_values`: Some(1), `max_size`: Some(24), added: 519, mode: `MaxEncodedLen`)
	fn submit_block_length_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65`
		//  Estimated: `1509`
		// Minimum execution time: 23_612_000 picoseconds.
		Weight::from_parts(23_950_000, 1509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// The range of component `i` is `[1, 524288]`.
	fn submit_data(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_847_000 picoseconds.
		Weight::from_parts(9_725_789, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_150, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// The range of component `i` is `[0, 524288]`.
	fn data_root(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_576_000 picoseconds.
		Weight::from_parts(1_619_000, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(4_729, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[0, 2097152]`.
	fn data_root_batch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_148_000 picoseconds.
		Weight::from_parts(1_173_000, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(4_730, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_32(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 460_453_000 picoseconds.
		Weight::from_parts(461_884_000, 0)
			// Standard Error: 42_886_657
			.saturating_add(Weight::from_parts(237_015_218, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_64(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 460_443_000 picoseconds.
		Weight::from_parts(462_479_000, 0)
			// Standard Error: 44_536_199
			.saturating_add(Weight::from_parts(2_210_568_345, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_128(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 461_118_000 picoseconds.
		Weight::from_parts(462_777_000, 0)
			// Standard Error: 32_430_679
			.saturating_add(Weight::from_parts(2_458_828_176, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_256(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 461_718_000 picoseconds.
		Weight::from_parts(32_423_258_650, 0)
			// Standard Error: 51_774_746
			.saturating_add(Weight::from_parts(2_635_132_917, 0).saturating_mul(i.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `DataAvailability::AppKeys` (r:1 w:1)
	/// Proof: `DataAvailability::AppKeys` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `DataAvailability::NextAppId` (r:1 w:1)
	/// Proof: `DataAvailability::NextAppId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn create_application_key() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `117`
		//  Estimated: `3583`
		// Minimum execution time: 28_304_000 picoseconds.
		Weight::from_parts(29_864_000, 3583)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `System::DynamicBlockLength` (r:1 w:1)
	/// Proof: `System::DynamicBlockLength` (`max_values`: Some(1), `max_size`: Some(24), added: 519, mode: `MaxEncodedLen`)
	fn submit_block_length_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `65`
		//  Estimated: `1509`
		// Minimum execution time: 23_612_000 picoseconds.
		Weight::from_parts(23_950_000, 1509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// The range of component `i` is `[1, 524288]`.
	fn submit_data(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 14_847_000 picoseconds.
		Weight::from_parts(9_725_789, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_150, 0).saturating_mul(i.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// The range of component `i` is `[0, 524288]`.
	fn data_root(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_576_000 picoseconds.
		Weight::from_parts(1_619_000, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(4_729, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[0, 2097152]`.
	fn data_root_batch(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_148_000 picoseconds.
		Weight::from_parts(1_173_000, 0)
			// Standard Error: 1
			.saturating_add(Weight::from_parts(4_730, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_32(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 460_453_000 picoseconds.
		Weight::from_parts(461_884_000, 0)
			// Standard Error: 42_886_657
			.saturating_add(Weight::from_parts(237_015_218, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_64(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 460_443_000 picoseconds.
		Weight::from_parts(462_479_000, 0)
			// Standard Error: 44_536_199
			.saturating_add(Weight::from_parts(2_210_568_345, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_128(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 461_118_000 picoseconds.
		Weight::from_parts(462_777_000, 0)
			// Standard Error: 32_430_679
			.saturating_add(Weight::from_parts(2_458_828_176, 0).saturating_mul(i.into()))
	}
	/// The range of component `i` is `[32, 1024]`.
	fn commitment_builder_256(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 461_718_000 picoseconds.
		Weight::from_parts(32_423_258_650, 0)
			// Standard Error: 51_774_746
			.saturating_add(Weight::from_parts(2_635_132_917, 0).saturating_mul(i.into()))
	}
}
