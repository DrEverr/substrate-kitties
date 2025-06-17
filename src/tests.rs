// Tests for the Kitties Pallet.
//
// Normally this file would be split into two parts: `mock.rs` and `tests.rs`.
// The `mock.rs` file would contain all the setup code for our `TestRuntime`.
// Then `tests.rs` would only have the tests for our pallet.
// However, to minimize the project, these have been combined into this single file.
//
// Learn more about creating tests for Pallets:
// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html

// This flag tells rust to only run this file when running `cargo test`.
#![cfg(test)]

use crate as pallet_kitties;
use crate::*;
use frame::deps::frame_support::runtime;
use frame::deps::sp_io;
use frame::runtime::prelude::*;
use frame::testing_prelude::*;
use frame::traits::Len;
use frame::traits::fungible::*;

type Balance = u64;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

const ALICE: u64 = 1;
const BOB: u64 = 2;
const DEFAULT_KITTY: Kitty<TestRuntime> = Kitty { dna: [0u8; 32], owner: 0 };

#[runtime]
mod runtime {
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeTask,
		RuntimeHoldReason,
		RuntimeFreezeReason
	)]
	#[runtime::runtime]
	/// The "test runtime" that represents the state transition function for our blockchain.
	///
	/// The runtime is composed of individual modules called "pallets", which you find see below.
	/// Each pallet has its own logic and storage, all of which can be combined together.
	pub struct TestRuntime;

	/// System: Mandatory system pallet that should always be included in a FRAME runtime.
	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<TestRuntime>;

	/// PalletBalances: Manages your blockchain's native currency. (i.e. DOT on Polkadot)
	#[runtime::pallet_index(1)]
	pub type PalletBalances = pallet_balances::Pallet<TestRuntime>;

	/// PalletKitties: The pallet you are building in this tutorial!
	#[runtime::pallet_index(2)]
	pub type PalletKitties = pallet_kitties::Pallet<TestRuntime>;
}

// Normally `System` would have many more configurations, but you can see that we use some macro
// magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

// Normally `pallet_balances` would have many more configurations, but you can see that we use some
// macro magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = Balance;
}

// This is the configuration of our Pallet! If you make changes to the pallet's `trait Config`, you
// will also need to update this configuration to represent that.
impl pallet_kitties::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
}

// We need to run most of our tests using this function: `new_test_ext().execute_with(|| { ... });`
// It simulates the blockchain database backend for our tests.
// If you forget to include this and try to access your Pallet storage, you will get an error like:
// "`get_version_1` called outside of an Externalities-provided environment."
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<TestRuntime>::default()
		.build_storage()
		.unwrap()
		.into()
}

#[test]
fn starting_template_is_sane() {
	new_test_ext().execute_with(|| {
		let event = Event::<TestRuntime>::Created { owner: ALICE };
		let _runtime_event: RuntimeEvent = event.into();
		let _call = Call::<TestRuntime>::create_kitty {};
		let result = PalletKitties::create_kitty(RuntimeOrigin::signed(BOB));
		assert_ok!(result);
	});
}

#[test]
fn system_and_balances_work() {
	// This test will just sanity check that we can access `System` and `PalletBalances`.
	new_test_ext().execute_with(|| {
		// We often need to add some balance to a user to test features which needs tokens.
		assert_ok!(PalletBalances::mint_into(&ALICE, 100));
		assert_ok!(PalletBalances::mint_into(&BOB, 100));
	});
}

#[test]
fn create_kitty_checks_signed() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_noop!(PalletKitties::create_kitty(RuntimeOrigin::none()), DispatchError::BadOrigin);
	});
}

#[test]
fn create_kitty_emits_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		System::assert_last_event(Event::<TestRuntime>::Created { owner: 1 }.into());
	})
}

#[test]
fn count_for_kitties_created_correctly() {
	new_test_ext().execute_with(|| {
		assert_eq!(CountForKitties::<TestRuntime>::get(), 0);
		CountForKitties::<TestRuntime>::set(1337u32);
		CountForKitties::<TestRuntime>::put(1337u32);
	})
}

#[test]
fn mint_increments_count_for_kitty() {
	new_test_ext().execute_with(|| {
		assert_eq!(CountForKitties::<TestRuntime>::get(), 0);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_eq!(CountForKitties::<TestRuntime>::get(), 1);
	})
}

#[test]
fn mint_errors_when_overflow() {
	new_test_ext().execute_with(|| {
		CountForKitties::<TestRuntime>::set(u32::MAX);
		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(1)),
			Error::<TestRuntime>::TooManyKitties
		);
	})
}

#[test]
fn kitties_map_created_correctly() {
	new_test_ext().execute_with(|| {
		let zero_key = [0u8; 32];
		assert!(!Kitties::<TestRuntime>::contains_key(zero_key));
		Kitties::<TestRuntime>::insert(zero_key, DEFAULT_KITTY);
		assert!(Kitties::<TestRuntime>::contains_key(zero_key));
	})
}

#[test]
fn create_kitty_adds_to_map() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 1);
	})
}

#[test]
fn cannot_mint_duplicate_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(ALICE, [0u8; 32]));
		assert_noop!(PalletKitties::mint(BOB, [0u8; 32]), Error::<TestRuntime>::DuplicateKitty);
	})
}

#[test]
fn kitty_struct_has_expected_traits() {
	new_test_ext().execute_with(|| {
		let kitty = DEFAULT_KITTY;
		let bytes = kitty.encode();
		let _decoded_kitty = Kitty::<TestRuntime>::decode(&mut &bytes[..]).unwrap();
		assert!(Kitty::<TestRuntime>::max_encoded_len() > 0);
		let _info = Kitty::<TestRuntime>::type_info();
	})
}

#[test]
fn mint_stores_owner_in_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(1337, [42u8; 32]));
		let kitty = Kitties::<TestRuntime>::get([42u8; 32]).unwrap();
		assert_eq!(kitty.owner, 1337);
		assert_eq!(kitty.dna, [42u8; 32]);
	})
}

#[test]
fn generate_dna_works() {
	new_test_ext().execute_with(|| {
		let dna = PalletKitties::gen_dna();
		assert_eq!(
			dna,
			[
				56, 98, 123, 124, 46, 238, 69, 214, 179, 29, 139, 103, 191, 184, 64, 133, 101, 187,
				84, 141, 18, 85, 176, 132, 68, 181, 190, 8, 83, 89, 192, 131
			]
		);
	})
}

#[test]
fn create_kitty_makes_unique_kitties() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(BOB)));
		assert_eq!(CountForKitties::<TestRuntime>::get(), 2);
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 2);
	})
}

#[test]
fn create_kitty_makes_unique_kitties_same_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_eq!(CountForKitties::<TestRuntime>::get(), 2);
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 2);
	})
}

#[test]
fn create_kitty_adds_to_owned() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(BOB)));
		assert_eq!(CountForKitties::<TestRuntime>::get(), 2);
		assert_eq!(KittiesOwned::<TestRuntime>::get(&ALICE).len(), 1);
		assert_eq!(KittiesOwned::<TestRuntime>::get(&BOB).len(), 1);
	})
}

#[test]
fn kitties_owned_created_correctly() {
	new_test_ext().execute_with(|| {
		assert_eq!(KittiesOwned::<TestRuntime>::get(&ALICE).len(), 0);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_eq!(KittiesOwned::<TestRuntime>::get(&ALICE).iter().count(), 2);
	});
}

#[test]
fn create_kitty_too_many_owned_error() {
	new_test_ext().execute_with(|| {
		let mut full_vec: BoundedVec<[u8; 32], ConstU32<100>> = BoundedVec::new();
		for _ in 0..100 {
			full_vec.try_push([0u8; 32]).unwrap();
		}
		KittiesOwned::<TestRuntime>::set(&ALICE, full_vec.into());
		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)),
			Error::<TestRuntime>::TooManyKittiesOwned
		);
	})
}

#[test]
fn transfer_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_ok!(PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, kitty_id));
		System::assert_last_event(
			Event::<TestRuntime>::Transferred { from: ALICE, to: BOB, kitty: kitty_id }.into(),
		);
	})
}

#[test]
fn do_transfer_kitty_transfer_to_self_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_noop!(
			PalletKitties::do_transfer(ALICE, ALICE, kitty_id),
			Error::<TestRuntime>::TransferToSelf
		);
	})
}

#[test]
fn do_transfer_kitty_not_owned_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_noop!(
			PalletKitties::do_transfer(BOB, ALICE, kitty_id),
			Error::<TestRuntime>::NotOwned
		);
	})
}

#[test]
fn do_transfer_kitty_no_kitty_error() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_noop!(
			PalletKitties::do_transfer(ALICE, BOB, [1; 32]),
			Error::<TestRuntime>::NoKitty
		);
	})
}

#[test]
fn do_transfer_kitty() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![kitty_id]);
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![]);
		assert_ok!(PalletKitties::do_transfer(ALICE, BOB, kitty_id));
		System::assert_last_event(
			Event::<TestRuntime>::Transferred { from: ALICE, to: BOB, kitty: kitty_id }.into(),
		);
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![]);
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![kitty_id]);
		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		assert_eq!(kitty.owner, BOB);
	})
}
