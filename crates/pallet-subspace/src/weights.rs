
//! Autogenerated weights for pallet_subspace
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_subspace
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./crates/pallet-subspace/src/weights.rs
// --template
// ./frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_subspace.
pub trait WeightInfo {
	fn report_equivocation() -> Weight;
	fn store_segment_headers(x: u32, ) -> Weight;
	fn enable_solution_range_adjustment() -> Weight;
	fn vote() -> Weight;
	fn enable_rewards() -> Weight;
	fn enable_storage_access() -> Weight;
	fn enable_authoring_by_anyone() -> Weight;
}

/// Weights for pallet_subspace using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: OffencesSubspace ReportsByKindIndex (r:1 w:1)
	/// Proof Skipped: OffencesSubspace ReportsByKindIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: OffencesSubspace ConcurrentReportsIndex (r:1 w:1)
	/// Proof Skipped: OffencesSubspace ConcurrentReportsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: OffencesSubspace Reports (r:1 w:1)
	/// Proof Skipped: OffencesSubspace Reports (max_values: None, max_size: None, mode: Measured)
	/// Storage: Subspace BlockList (r:0 w:1)
	/// Proof Skipped: Subspace BlockList (max_values: None, max_size: None, mode: Measured)
	fn report_equivocation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `10563`
		// Minimum execution time: 22_000_000 picoseconds.
		Weight::from_parts(23_000_000, 10563)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Subspace SegmentCommitment (r:1023 w:1023)
	/// Proof Skipped: Subspace SegmentCommitment (max_values: None, max_size: None, mode: Measured)
	/// Storage: Subspace CounterForSegmentCommitment (r:1 w:1)
	/// Proof: Subspace CounterForSegmentCommitment (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 1024]`.
	fn store_segment_headers(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `4060 + x * (2475 ±0)`
		// Minimum execution time: 440_000_000 picoseconds.
		Weight::from_parts(432_662_004, 4060)
			// Standard Error: 158_785
			.saturating_add(Weight::from_parts(436_196_449, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(x.into()))
	}
	/// Storage: Subspace ShouldAdjustSolutionRange (r:1 w:1)
	/// Proof Skipped: Subspace ShouldAdjustSolutionRange (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Subspace SolutionRanges (r:1 w:1)
	/// Proof Skipped: Subspace SolutionRanges (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Subspace NextSolutionRangeOverride (r:0 w:1)
	/// Proof Skipped: Subspace NextSolutionRangeOverride (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_solution_range_adjustment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `4647`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(12_000_000, 4647)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Subspace BlockList (r:1 w:0)
	/// Proof Skipped: Subspace BlockList (max_values: None, max_size: None, mode: Measured)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `3513`
		// Minimum execution time: 1_288_000_000 picoseconds.
		Weight::from_parts(1_297_000_000, 3513)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Subspace EnableRewards (r:1 w:1)
	/// Proof Skipped: Subspace EnableRewards (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `1533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Subspace IsStorageAccessEnabled (r:0 w:1)
	/// Proof Skipped: Subspace IsStorageAccessEnabled (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_storage_access() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000 picoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Subspace RootPlotPublicKey (r:1 w:0)
	/// Proof Skipped: Subspace RootPlotPublicKey (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Subspace AllowAuthoringByAnyone (r:0 w:1)
	/// Proof Skipped: Subspace AllowAuthoringByAnyone (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_authoring_by_anyone() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `3114`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(7_000_000, 3114)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: OffencesSubspace ReportsByKindIndex (r:1 w:1)
	/// Proof Skipped: OffencesSubspace ReportsByKindIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: OffencesSubspace ConcurrentReportsIndex (r:1 w:1)
	/// Proof Skipped: OffencesSubspace ConcurrentReportsIndex (max_values: None, max_size: None, mode: Measured)
	/// Storage: OffencesSubspace Reports (r:1 w:1)
	/// Proof Skipped: OffencesSubspace Reports (max_values: None, max_size: None, mode: Measured)
	/// Storage: Subspace BlockList (r:0 w:1)
	/// Proof Skipped: Subspace BlockList (max_values: None, max_size: None, mode: Measured)
	fn report_equivocation() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `10563`
		// Minimum execution time: 22_000_000 picoseconds.
		Weight::from_parts(23_000_000, 10563)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Subspace SegmentCommitment (r:1023 w:1023)
	/// Proof Skipped: Subspace SegmentCommitment (max_values: None, max_size: None, mode: Measured)
	/// Storage: Subspace CounterForSegmentCommitment (r:1 w:1)
	/// Proof: Subspace CounterForSegmentCommitment (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[1, 1024]`.
	fn store_segment_headers(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `4060 + x * (2475 ±0)`
		// Minimum execution time: 440_000_000 picoseconds.
		Weight::from_parts(432_662_004, 4060)
			// Standard Error: 158_785
			.saturating_add(Weight::from_parts(436_196_449, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(x.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(x.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(x.into()))
	}
	/// Storage: Subspace ShouldAdjustSolutionRange (r:1 w:1)
	/// Proof Skipped: Subspace ShouldAdjustSolutionRange (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Subspace SolutionRanges (r:1 w:1)
	/// Proof Skipped: Subspace SolutionRanges (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Subspace NextSolutionRangeOverride (r:0 w:1)
	/// Proof Skipped: Subspace NextSolutionRangeOverride (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_solution_range_adjustment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `4647`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(12_000_000, 4647)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Subspace BlockList (r:1 w:0)
	/// Proof Skipped: Subspace BlockList (max_values: None, max_size: None, mode: Measured)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `3513`
		// Minimum execution time: 1_288_000_000 picoseconds.
		Weight::from_parts(1_297_000_000, 3513)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Subspace EnableRewards (r:1 w:1)
	/// Proof Skipped: Subspace EnableRewards (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `1533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 1533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Subspace IsStorageAccessEnabled (r:0 w:1)
	/// Proof Skipped: Subspace IsStorageAccessEnabled (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_storage_access() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_000_000 picoseconds.
		Weight::from_parts(3_000_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Subspace RootPlotPublicKey (r:1 w:0)
	/// Proof Skipped: Subspace RootPlotPublicKey (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Subspace AllowAuthoringByAnyone (r:0 w:1)
	/// Proof Skipped: Subspace AllowAuthoringByAnyone (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_authoring_by_anyone() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `48`
		//  Estimated: `3114`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(7_000_000, 3114)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
