// Copyright 2023-2024 Freeverse.io
// This file is part of LAOS.

// LAOS is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// LAOS is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with LAOS.  If not, see <http://www.gnu.org/licenses/>.

//! Benchmarking setup for pallet-living-assets-evolution
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::pallet::Pallet;
#[allow(unused)]
// use pallet_vesting::Pallet as PalletVesting;
use fp_evm::Transfer;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use pallet_evm::{Context, ExitError, ExitReason, Log, PrecompileHandle};
use pallet_vesting::Pallet as PalletVesting;
use precompile_utils::prelude::Address;
use sp_core::{Get, H160, H256, U256};
use sp_runtime::traits::{CheckedMul, Convert};
use sp_std::{vec, vec::Vec};

pub struct MockHandle {
	pub input: Vec<u8>,
	pub gas_limit: Option<u64>,
	pub context: Context,
	pub is_static: bool,
	pub gas_used: u64,
	pub logs: Vec<Log>,
	pub code_address: H160,
}

impl MockHandle {
	pub fn new(caller: H160) -> Self {
		Self {
			input: vec![],
			gas_limit: None,
			context: Context { address: H160::zero(), caller, apparent_value: U256::zero() },
			is_static: false,
			gas_used: 0,
			logs: vec![],
			code_address: H160::zero(),
		}
	}
}

impl PrecompileHandle for MockHandle {
	/// Perform subcall in provided context.
	/// Precompile specifies in which context the subcall is executed.
	fn call(
		&mut self,
		_: H160,
		_: Option<Transfer>,
		_: Vec<u8>,
		_: Option<u64>,
		_: bool,
		_: &Context,
	) -> (ExitReason, Vec<u8>) {
		unimplemented!()
	}

	fn record_cost(&mut self, cost: u64) -> Result<(), ExitError> {
		self.gas_used += cost;
		Ok(())
	}

	fn record_external_cost(&mut self, _: Option<u64>, _: Option<u64>) -> Result<(), ExitError> {
		Ok(())
	}

	fn refund_external_cost(&mut self, _: Option<u64>, _: Option<u64>) {}

	fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) -> Result<(), ExitError> {
		let log = Log { address, topics, data };
		self.logs.push(log);
		Ok(())
	}

	fn remaining_gas(&self) -> u64 {
		1000000000000
	}

	fn code_address(&self) -> H160 {
		self.code_address
	}

	fn input(&self) -> &[u8] {
		&self.input
	}

	fn context(&self) -> &Context {
		&self.context
	}

	fn is_static(&self) -> bool {
		self.is_static
	}

	fn gas_limit(&self) -> Option<u64> {
		self.gas_limit
	}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn precompile_vest() {
		let caller: T::AccountId = whitelisted_caller();
		let caller_origin = T::RuntimeOrigin::from(RawOrigin::from(Some(caller.clone())));
		let min_transfer = T::MinVestedTransfer::get();
		let _ = T::Currency::issue(min_transfer);
		T::Currency::make_free_balance_be(&caller, min_transfer);
		
		let target: T::AccountId = account("target", 0, 1);
		let mut handle = MockHandle::new(T::AccountIdToH160::convert(target.clone()));
		let target_lookup = T::Lookup::unlookup(target.clone());
		let starting_block = 0u32;
		let per_block = min_transfer;

		PalletVesting::<T>::vested_transfer(
			caller_origin,
			target_lookup,
			pallet_vesting::VestingInfo::new(min_transfer, per_block, starting_block.into()),
		)
		.unwrap();

		#[block]
		{
			VestingPrecompile::<T>::vest(&mut handle).unwrap();
		}
	}
}
