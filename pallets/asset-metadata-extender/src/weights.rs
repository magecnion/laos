
//! Autogenerated weights for `pallet_asset_metadata_extender`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `trujideb`, CPU: `12th Gen Intel(R) Core(TM) i5-12500H`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/laos
// benchmark
// pallet
// --steps=50
// --repeat=20
// --pallet=pallet-asset-metadata-extender
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=pallets/asset-metadata-extender/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_asset_metadata_extender`.
pub trait WeightInfo {
	fn precompile_extend(t: u32, u: u32, ) -> Weight;
	fn precompile_update(t: u32, u: u32, ) -> Weight;
	fn create_token_uri_extension(t: u32, u: u32, ) -> Weight;
	fn update_token_uri_extension(t: u32, u: u32, ) -> Weight;
}

/// Weights for `pallet_asset_metadata_extender` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ClaimersByLocationAndIndex` (r:0 w:1)
	/// Proof: `AssetMetadataExtender::ClaimersByLocationAndIndex` (`max_values`: None, `max_size`: Some(570), added: 3045, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn precompile_extend(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4545`
		// Minimum execution time: 15_167_000 picoseconds.
		Weight::from_parts(15_884_425, 4545)
			// Standard Error: 230
			.saturating_add(Weight::from_parts(1_745, 0).saturating_mul(t.into()))
			// Standard Error: 230
			.saturating_add(Weight::from_parts(10_833, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn precompile_update(_t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185 + u * (1 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 13_045_000 picoseconds.
		Weight::from_parts(15_048_370, 4545)
			// Standard Error: 138
			.saturating_add(Weight::from_parts(8_883, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ClaimersByLocationAndIndex` (r:0 w:1)
	/// Proof: `AssetMetadataExtender::ClaimersByLocationAndIndex` (`max_values`: None, `max_size`: Some(570), added: 3045, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn create_token_uri_extension(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4545`
		// Minimum execution time: 12_087_000 picoseconds.
		Weight::from_parts(13_170_500, 4545)
			// Standard Error: 136
			.saturating_add(Weight::from_parts(955, 0).saturating_mul(t.into()))
			// Standard Error: 136
			.saturating_add(Weight::from_parts(6_756, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn update_token_uri_extension(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185 + u * (1 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 10_389_000 picoseconds.
		Weight::from_parts(10_659_664, 4545)
			// Standard Error: 70
			.saturating_add(Weight::from_parts(818, 0).saturating_mul(t.into()))
			// Standard Error: 70
			.saturating_add(Weight::from_parts(6_654, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ClaimersByLocationAndIndex` (r:0 w:1)
	/// Proof: `AssetMetadataExtender::ClaimersByLocationAndIndex` (`max_values`: None, `max_size`: Some(570), added: 3045, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn precompile_extend(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4545`
		// Minimum execution time: 15_167_000 picoseconds.
		Weight::from_parts(15_884_425, 4545)
			// Standard Error: 230
			.saturating_add(Weight::from_parts(1_745, 0).saturating_mul(t.into()))
			// Standard Error: 230
			.saturating_add(Weight::from_parts(10_833, 0).saturating_mul(u.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn precompile_update(_t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185 + u * (1 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 13_045_000 picoseconds.
		Weight::from_parts(15_048_370, 4545)
			// Standard Error: 138
			.saturating_add(Weight::from_parts(8_883, 0).saturating_mul(u.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ExtensionsCounter` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::ExtensionsCounter` (`max_values`: None, `max_size`: Some(534), added: 3009, mode: `MaxEncodedLen`)
	/// Storage: `AssetMetadataExtender::ClaimersByLocationAndIndex` (r:0 w:1)
	/// Proof: `AssetMetadataExtender::ClaimersByLocationAndIndex` (`max_values`: None, `max_size`: Some(570), added: 3045, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn create_token_uri_extension(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `4545`
		// Minimum execution time: 12_087_000 picoseconds.
		Weight::from_parts(13_170_500, 4545)
			// Standard Error: 136
			.saturating_add(Weight::from_parts(955, 0).saturating_mul(t.into()))
			// Standard Error: 136
			.saturating_add(Weight::from_parts(6_756, 0).saturating_mul(u.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (r:1 w:1)
	/// Proof: `AssetMetadataExtender::TokenUrisByClaimerAndLocation` (`max_values`: None, `max_size`: Some(1080), added: 3555, mode: `MaxEncodedLen`)
	/// The range of component `t` is `[0, 512]`.
	/// The range of component `u` is `[0, 512]`.
	fn update_token_uri_extension(t: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185 + u * (1 ±0)`
		//  Estimated: `4545`
		// Minimum execution time: 10_389_000 picoseconds.
		Weight::from_parts(10_659_664, 4545)
			// Standard Error: 70
			.saturating_add(Weight::from_parts(818, 0).saturating_mul(t.into()))
			// Standard Error: 70
			.saturating_add(Weight::from_parts(6_654, 0).saturating_mul(u.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}