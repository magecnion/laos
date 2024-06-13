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

//! Living assets precompile tests.

use super::*;
use crate::mock::*;
use core::str::FromStr;
use fp_evm::Log;
use pallet_evm::AccountCodes;
use precompile_utils::{
	prelude::log2,
	testing::{Alice, Precompile1, PrecompileTesterExt},
};
use sp_core::{H160, H256, U256};

/// Get precompiles from the mock.
fn precompiles() -> LaosPrecompiles<Test> {
	PrecompilesInstance::get()
}

#[test]
fn check_log_selectors() {
	assert_eq!(
		hex::encode(SELECTOR_LOG_NEW_COLLECTION),
		"5b84d9550adb7000df7bee717735ecd3af48ea3f66c6886d52e8227548fb228c"
	);
}

#[test]
fn selectors() {
	assert!(PrecompileCall::create_collection_selectors().contains(&0x2069E953));
}

#[test]
fn create_collection_returns_address() {
	new_test_ext().execute_with(|| {
		let expected_collection_address =
			H160::from_str("fffffffffffffffffffffffe0000000000000000").unwrap();
		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::create_collection { owner: Address(Alice.into()) },
			)
			.execute_returns(Address(expected_collection_address.into()));
	})
}

#[test]
fn create_collection_should_generate_log() {
	new_test_ext().execute_with(|| {
		let expected_collection_address =
			H160::from_str("fffffffffffffffffffffffe0000000000000000").unwrap();

		precompiles()
			.prepare_test(
				Alice,
				Precompile1,
				PrecompileCall::create_collection { owner: Address(Alice.into()) },
			)
			.expect_log(log2(
				Precompile1,
				SELECTOR_LOG_NEW_COLLECTION,
				Alice,
				solidity::encode_event_data(Address(expected_collection_address.into())),
			))
			.execute_some();
	});
}

// #[test]
// fn create_collection_on_mock_with_nonzero_value_fails() {
// 	new_test_ext().execute_with(|| {
// 		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
// 			.write(Address(H160([1u8; 20])))
// 			.build();

// 		precompiles()
// 			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
// 			.with_value(U256::from(1))
// 			.execute_reverts(|r| r == b"function is not payable");
// 	});
// }

// #[test]
// fn create_collection_assign_collection_to_caller() {
// 	new_test_ext().execute_with(|| {
// 		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
// 			.write(Address(H160([1u8; 20])))
// 			.build();

// 		let expected_address = "fffffffffffffffffffffffe0000000000000000";
// 		// output is padded with 12 bytes of zeros
// 		let expected_output =
// 			H256::from_str(format!("000000000000000000000000{}", expected_address).as_str())
// 				.unwrap();

// 		precompiles()
// 			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
// 			.execute_returns(expected_output);

// 		assert_eq!(LaosEvolution::<Test>::collection_owner(0), Some(H160([1u8; 20])));
// 	});
// }

// #[test]
// fn create_collection_inserts_bytecode_to_address() {
// 	new_test_ext().execute_with(|| {
// 		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
// 			.write(Address(H160([1u8; 20])))
// 			.build();

// 		let expected_address = "fffffffffffffffffffffffe0000000000000000";
// 		// output is padded with 12 bytes of zeros
// 		let expected_output =
// 			H256::from_str(format!("000000000000000000000000{}", expected_address).as_str())
// 				.unwrap();

// 		precompiles()
// 			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
// 			.execute_returns(expected_output);

// 		let collection_address = &H160::from_str(expected_address).unwrap();
// 		// Address is not empty
// 		assert!(!Evm::<Test>::is_account_empty(collection_address));

// 		// Address has correct code
// 		assert!(AccountCodes::<Test>::get(collection_address) == REVERT_BYTECODE);
// 	});
// }

// #[test]
// fn test_expected_cost_create_collection() {
// 	new_test_ext().execute_with(|| {
// 		let input = EvmDataWriter::new_with_selector(Action::CreateCollection)
// 			.write(Address(H160([1u8; 20])))
// 			.build();

// 		// Expected weight of the precompile call implementation.
// 		// Since benchmarking precompiles is not supported yet, we are benchmarking
// 		// functions that precompile calls internally.
// 		//
// 		// The weight of this precompile call is calculated as:
// 		// `create_collection` weight + insert account bytecode for the collection + log costs
// 		precompiles()
// 			.prepare_test(H160([1u8; 20]), H160(PRECOMPILE_ADDRESS), input)
// 			.expect_cost(486001381) //[`WeightToGas`] set to 1:1 in mock
// 			.execute_some();
// 	})
// }
