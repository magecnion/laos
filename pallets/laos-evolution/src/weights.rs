
//! Autogenerated weights for `pallet_laos_evolution`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `titan`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/laos
// benchmark
// pallet
// --pallet
// pallet-laos-evolution
// --extrinsic=*
// --output
// pallets/laos-evolution/src/weights.rs
// --execution
// wasm
// --wasm-execution=compiled
// --steps
// 50
// --repeat
// 20
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_laos_evolution`.
pub trait WeightInfo {
	fn precompile_discriminant() -> Weight;
	fn precompile_create_collection() -> Weight;
	fn precompile_mint(s: u32, ) -> Weight;
	fn precompile_evolve(s: u32, ) -> Weight;
	fn precompile_transfer_ownership() -> Weight;
	fn precompile_owner() -> Weight;
	fn precompile_token_uri() -> Weight;
	fn create_collection() -> Weight;
	fn mint_with_external_uri(s: u32, ) -> Weight;
	fn evolve_with_external_uri(s: u32, ) -> Weight;
	fn enable_public_minting() -> Weight;
	fn disable_public_minting() -> Weight;
	fn transfer_ownership() -> Weight;
}

/// Weights for `pallet_laos_evolution` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn precompile_discriminant() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 266_000 picoseconds.
		Weight::from_parts(319_000, 0)
	}
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `EVM::Suicided` (r:1 w:0)
	/// Proof: `EVM::Suicided` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `EVM::AccountCodes` (r:1 w:1)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountCodesMetadata` (r:0 w:1)
	/// Proof: `EVM::AccountCodesMetadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `375`
		//  Estimated: `3840`
		// Minimum execution time: 30_193_000 picoseconds.
		Weight::from_parts(32_520_000, 3840)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn precompile_mint(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `4051`
		// Minimum execution time: 20_807_000 picoseconds.
		Weight::from_parts(22_501_463, 4051)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn precompile_evolve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264`
		//  Estimated: `4051`
		// Minimum execution time: 18_884_000 picoseconds.
		Weight::from_parts(21_037_684, 4051)
			// Standard Error: 204
			.saturating_add(Weight::from_parts(820, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 11_222_000 picoseconds.
		Weight::from_parts(11_807_000, 3509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 6_686_000 picoseconds.
		Weight::from_parts(7_280_000, 3509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:0)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	fn precompile_token_uri() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `4051`
		// Minimum execution time: 9_640_000 picoseconds.
		Weight::from_parts(10_289_000, 4051)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1493`
		// Minimum execution time: 12_747_000 picoseconds.
		Weight::from_parts(13_178_000, 1493)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn mint_with_external_uri(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `4051`
		// Minimum execution time: 19_859_000 picoseconds.
		Weight::from_parts(23_616_615, 4051)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn evolve_with_external_uri(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264`
		//  Estimated: `4051`
		// Minimum execution time: 18_351_000 picoseconds.
		Weight::from_parts(20_489_182, 4051)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn enable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 13_639_000 picoseconds.
		Weight::from_parts(14_059_000, 3509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn disable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172`
		//  Estimated: `3509`
		// Minimum execution time: 13_650_000 picoseconds.
		Weight::from_parts(14_150_000, 3509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 11_500_000 picoseconds.
		Weight::from_parts(11_925_000, 3509)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	fn precompile_discriminant() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 266_000 picoseconds.
		Weight::from_parts(319_000, 0)
	}
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `EVM::Suicided` (r:1 w:0)
	/// Proof: `EVM::Suicided` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `EVM::AccountCodes` (r:1 w:1)
	/// Proof: `EVM::AccountCodes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(116), added: 2591, mode: `MaxEncodedLen`)
	/// Storage: `EVM::AccountCodesMetadata` (r:0 w:1)
	/// Proof: `EVM::AccountCodesMetadata` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `375`
		//  Estimated: `3840`
		// Minimum execution time: 30_193_000 picoseconds.
		Weight::from_parts(32_520_000, 3840)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn precompile_mint(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `4051`
		// Minimum execution time: 20_807_000 picoseconds.
		Weight::from_parts(22_501_463, 4051)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn precompile_evolve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264`
		//  Estimated: `4051`
		// Minimum execution time: 18_884_000 picoseconds.
		Weight::from_parts(21_037_684, 4051)
			// Standard Error: 204
			.saturating_add(Weight::from_parts(820, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 11_222_000 picoseconds.
		Weight::from_parts(11_807_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn precompile_owner() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 6_686_000 picoseconds.
		Weight::from_parts(7_280_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:0)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	fn precompile_token_uri() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `306`
		//  Estimated: `4051`
		// Minimum execution time: 9_640_000 picoseconds.
		Weight::from_parts(10_289_000, 4051)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionCounter` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionCounter` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionOwner` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn create_collection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `1493`
		// Minimum execution time: 12_747_000 picoseconds.
		Weight::from_parts(13_178_000, 1493)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn mint_with_external_uri(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `4051`
		// Minimum execution time: 19_859_000 picoseconds.
		Weight::from_parts(23_616_615, 4051)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::TokenURI` (r:1 w:1)
	/// Proof: `LaosEvolution::TokenURI` (`max_values`: None, `max_size`: Some(586), added: 3061, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 512]`.
	fn evolve_with_external_uri(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `264`
		//  Estimated: `4051`
		// Minimum execution time: 18_351_000 picoseconds.
		Weight::from_parts(20_489_182, 4051)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn enable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 13_639_000 picoseconds.
		Weight::from_parts(14_059_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:0)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `LaosEvolution::CollectionPublicMintingEnabled` (r:0 w:1)
	/// Proof: `LaosEvolution::CollectionPublicMintingEnabled` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	fn disable_public_minting() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `172`
		//  Estimated: `3509`
		// Minimum execution time: 13_650_000 picoseconds.
		Weight::from_parts(14_150_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `LaosEvolution::CollectionOwner` (r:1 w:1)
	/// Proof: `LaosEvolution::CollectionOwner` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `139`
		//  Estimated: `3509`
		// Minimum execution time: 11_500_000 picoseconds.
		Weight::from_parts(11_925_000, 3509)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}