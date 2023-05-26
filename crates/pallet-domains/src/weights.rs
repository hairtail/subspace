
//! TODO: regenerate this file once the standard machine for subspace node is determined.
//! Autogenerated weights for pallet_domains
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-08, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/subspace-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_domains
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./crates/pallet-domains/src/weights.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_domains.
pub trait WeightInfo {
	fn submit_system_bundle(x: u32, ) -> Weight;
	fn submit_core_bundle() -> Weight;
	fn submit_system_domain_invalid_state_transition_proof() -> Weight;
}

/// Weights for pallet_domains using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Receipts OldestReceiptNumber (r:1 w:1)
	/// Proof Skipped: Receipts OldestReceiptNumber (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts HeadReceiptNumber (r:1 w:1)
	/// Proof Skipped: Receipts HeadReceiptNumber (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts Receipts (r:256 w:512)
	/// Proof Skipped: Receipts Receipts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts ReceiptVotes (r:768 w:512)
	/// Proof Skipped: Receipts ReceiptVotes (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts PrimaryBlockHash (r:256 w:256)
	/// Proof Skipped: Receipts PrimaryBlockHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts StateRoots (r:255 w:511)
	/// Proof Skipped: Receipts StateRoots (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 256]`.
	fn submit_system_bundle(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `30097 + x * (236 ±0)`
		//  Estimated: `156064 + x * (16435 ±2)`
		// Minimum execution time: 100_000_000 picoseconds.
		Weight::from_parts(114_000_000, 156064)
			// Standard Error: 83_023
			.saturating_add(Weight::from_parts(57_171_657, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 16435).saturating_mul(x.into()))
	}
	fn submit_core_bundle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
	}
	/// Storage: Receipts HeadReceiptNumber (r:1 w:1)
	/// Proof Skipped: Receipts HeadReceiptNumber (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts PrimaryBlockHash (r:256 w:0)
	/// Proof Skipped: Receipts PrimaryBlockHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts ReceiptVotes (r:510 w:255)
	/// Proof Skipped: Receipts ReceiptVotes (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts Receipts (r:255 w:255)
	/// Proof Skipped: Receipts Receipts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts StateRoots (r:100 w:255)
	/// Proof Skipped: Receipts StateRoots (max_values: None, max_size: None, mode: Measured)
	fn submit_system_domain_invalid_state_transition_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100459`
		//  Estimated: `3284195`
		// Minimum execution time: 7_251_000_000 picoseconds.
		Weight::from_parts(7_404_000_000, 3284195)
			.saturating_add(T::DbWeight::get().reads(1122_u64))
			.saturating_add(T::DbWeight::get().writes(766_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Receipts OldestReceiptNumber (r:1 w:1)
	/// Proof Skipped: Receipts OldestReceiptNumber (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts HeadReceiptNumber (r:1 w:1)
	/// Proof Skipped: Receipts HeadReceiptNumber (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts Receipts (r:256 w:512)
	/// Proof Skipped: Receipts Receipts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts ReceiptVotes (r:768 w:512)
	/// Proof Skipped: Receipts ReceiptVotes (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts PrimaryBlockHash (r:256 w:256)
	/// Proof Skipped: Receipts PrimaryBlockHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts StateRoots (r:255 w:511)
	/// Proof Skipped: Receipts StateRoots (max_values: None, max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 256]`.
	fn submit_system_bundle(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `30097 + x * (236 ±0)`
		//  Estimated: `156064 + x * (16435 ±2)`
		// Minimum execution time: 100_000_000 picoseconds.
		Weight::from_parts(114_000_000, 156064)
			// Standard Error: 83_023
			.saturating_add(Weight::from_parts(57_171_657, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((6_u64).saturating_mul(x.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((7_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 16435).saturating_mul(x.into()))
	}
	fn submit_core_bundle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_000_000 picoseconds.
		Weight::from_parts(12_000_000, 0)
	}
	/// Storage: Receipts HeadReceiptNumber (r:1 w:1)
	/// Proof Skipped: Receipts HeadReceiptNumber (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts PrimaryBlockHash (r:256 w:0)
	/// Proof Skipped: Receipts PrimaryBlockHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts ReceiptVotes (r:510 w:255)
	/// Proof Skipped: Receipts ReceiptVotes (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts Receipts (r:255 w:255)
	/// Proof Skipped: Receipts Receipts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Receipts StateRoots (r:100 w:255)
	/// Proof Skipped: Receipts StateRoots (max_values: None, max_size: None, mode: Measured)
	fn submit_system_domain_invalid_state_transition_proof() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100459`
		//  Estimated: `3284195`
		// Minimum execution time: 7_251_000_000 picoseconds.
		Weight::from_parts(7_404_000_000, 3284195)
			.saturating_add(RocksDbWeight::get().reads(1122_u64))
			.saturating_add(RocksDbWeight::get().writes(766_u64))
	}
}