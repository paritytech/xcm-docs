// Copyright Parity Technologies (UK) Ltd.
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
#![allow(dead_code)]
pub mod parachain;
pub mod relay_chain;

use core::{borrow::Borrow, marker::PhantomData};

use frame_support::{
	ensure,
	pallet_prelude::Weight,
	sp_tracing,
	traits::{GenesisBuild, ProcessMessageError},
};
use sp_core::blake2_256;
use xcm::prelude::*;
use xcm_executor::traits::{Convert, ShouldExecute};
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain, TestExt};

// Accounts
pub const ADMIN: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);
pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([1u8; 32]);
pub const BOB: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([2u8; 32]);

// Balances
pub type Balance = u128;
pub const UNITS: Balance = 10_000_000_000;
pub const CENTS: Balance = UNITS / 100; // 100_000_000
pub const INITIAL_BALANCE: u128 = 10 * UNITS;

decl_test_parachain! {
	pub struct ParaA {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(1),
	}
}

decl_test_parachain! {
	pub struct ParaB {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(2),
	}
}

decl_test_parachain! {
	pub struct ParaC {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::MsgQueue,
		DmpMessageHandler = parachain::MsgQueue,
		new_ext = para_ext(2),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		RuntimeCall = relay_chain::RuntimeCall,
		RuntimeEvent = relay_chain::RuntimeEvent,
		XcmConfig = relay_chain::XcmConfig,
		MessageQueue = relay_chain::MessageQueue,
		System = relay_chain::System,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		parachains = vec![
			(1, ParaA),
			(2, ParaB),
			(3, ParaC),
		],
	}
}

pub fn relay_sovereign_account_id() -> parachain::AccountId {
	let location = (Parent,);
	parachain::SovereignAccountOf::convert(location.into()).unwrap()
}

pub fn parachain_sovereign_account_id(para: u32) -> relay_chain::AccountId {
	let location = (Parachain(para),);
	relay_chain::SovereignAccountOf::convert(location.into()).unwrap()
}

pub fn parachain_account_sovereign_account_id(
	para: u32,
	who: sp_runtime::AccountId32,
) -> relay_chain::AccountId {
	let location = (
		Parachain(para),
		AccountId32 { network: Some(relay_chain::RelayNetwork::get()), id: who.into() },
	);
	relay_chain::SovereignAccountOf::convert(location.into()).unwrap()
}

pub fn sibling_sovereign_account_id(para: u32) -> parachain::AccountId {
	let location = (Parent, Parachain(para));
	parachain::SovereignAccountOf::convert(location.into()).unwrap()
}

pub fn sibling_account_sovereign_account_id(
	para: u32,
	who: sp_runtime::AccountId32,
) -> parachain::AccountId {
	let location = (Parent, Parachain(para), AccountId32 { network: None, id: who.into() });
	parachain::SovereignAccountOf::convert(location.into()).unwrap()
}

pub fn relay_account_sovereign_account_id(who: sp_runtime::AccountId32) -> parachain::AccountId {
	let location = (Parent, AccountId32 { network: None, id: who.into() });
	parachain::SovereignAccountOf::convert(location.into()).unwrap()
}

pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	use parachain::{MsgQueue, Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
	let other_para_ids = match para_id {
		1 => [2, 3],
		2 => [1, 3],
		3 => [1, 2],
		_ => panic!("No parachain exists with para_id = {para_id}"),
	};

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, INITIAL_BALANCE), (relay_sovereign_account_id(), INITIAL_BALANCE), (BOB, INITIAL_BALANCE)]
			.into_iter()
			.chain(other_para_ids.iter().map(
				// Initial balance of native token for ALICE on all sibling sovereign accounts
				|&para_id| (sibling_account_sovereign_account_id(para_id, ALICE), INITIAL_BALANCE),
			))
			.chain(other_para_ids.iter().map(
				// Initial balance of native token all sibling sovereign accounts
				|&para_id| (sibling_sovereign_account_id(para_id), INITIAL_BALANCE),
			))
			.collect(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	pallet_assets::GenesisConfig::<Runtime> {
		assets: vec![
			(0u128, ADMIN, false, 1u128), // Create derivative asset for relay's native token
		]
		.into_iter()
		.chain(other_para_ids.iter().map(|&para_id| (para_id as u128, ADMIN, false, 1u128))) // Derivative assets for the other parachains' native tokens
		.collect(),
		metadata: Default::default(),
		accounts: vec![
			(0u128, ALICE, INITIAL_BALANCE),
			(0u128, relay_sovereign_account_id(), INITIAL_BALANCE),
		]
		.into_iter()
		.chain(other_para_ids.iter().map(|&para_id| (para_id as u128, ALICE, INITIAL_BALANCE))) // Initial balance for derivatives of other parachains' tokens
		.chain(other_para_ids.iter().map(|&para_id| {
			(0u128, sibling_account_sovereign_account_id(para_id, ALICE), INITIAL_BALANCE)
		})) // Initial balance for sovereign accounts (for fee payment)
		.chain(
			other_para_ids
				.iter()
				.map(|&para_id| (0u128, sibling_sovereign_account_id(para_id), INITIAL_BALANCE)),
		) // Initial balance for sovereign accounts (for fee payment)
		.collect(),
	}
	.assimilate_storage(&mut t)
	.unwrap();


	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		sp_tracing::try_init_simple();
		System::set_block_number(1);
		MsgQueue::set_para_id(para_id.into());
	});
	ext
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use relay_chain::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![
			(ALICE, INITIAL_BALANCE),
			(parachain_sovereign_account_id(1), INITIAL_BALANCE),
			(parachain_sovereign_account_id(2), INITIAL_BALANCE),
			(parachain_sovereign_account_id(3), INITIAL_BALANCE),
			(parachain_account_sovereign_account_id(1, ALICE), INITIAL_BALANCE),
			(parachain_account_sovereign_account_id(2, ALICE), INITIAL_BALANCE),
			(parachain_account_sovereign_account_id(3, ALICE), INITIAL_BALANCE),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}

pub fn print_para_events() {
	use parachain::System;
	System::events().iter().for_each(|r| println!(">>> {:?}", r.event));
}

pub fn print_relay_events() {
	use relay_chain::System;
	System::events().iter().for_each(|r| println!(">>> {:?}", r.event));
}

pub type RelaychainPalletXcm = pallet_xcm::Pallet<relay_chain::Runtime>;
pub type ParachainPalletXcm = pallet_xcm::Pallet<parachain::Runtime>;
pub type RelaychainBalances = pallet_balances::Pallet<relay_chain::Runtime>;
pub type ParachainBalances = pallet_balances::Pallet<parachain::Runtime>;
pub type ParachainAssets = pallet_assets::Pallet<parachain::Runtime>;

/// Prefix for generating alias account for accounts coming  
/// from chains that use 32 byte long representations.
pub const FOREIGN_CHAIN_PREFIX_PARA_32: [u8; 37] = *b"ForeignChainAliasAccountPrefix_Para32";

/// Prefix for generating alias account for accounts coming  
/// from the relay chain using 32 byte long representations.
pub const FOREIGN_CHAIN_PREFIX_RELAY: [u8; 36] = *b"ForeignChainAliasAccountPrefix_Relay";

pub struct ForeignChainAliasAccount<AccountId>(PhantomData<AccountId>);
impl<AccountId: From<[u8; 32]> + Clone> Convert<MultiLocation, AccountId>
	for ForeignChainAliasAccount<AccountId>
{
	fn convert_ref(location: impl Borrow<MultiLocation>) -> Result<AccountId, ()> {
		let entropy = match location.borrow() {
			// Used on the relay chain for sending paras that use 32 byte accounts
			MultiLocation {
				parents: 0,
				interior: X2(Parachain(para_id), AccountId32 { id, .. }),
			} => ForeignChainAliasAccount::<AccountId>::from_para_32(para_id, id, 0),

			// Used on para-chain for sending paras that use 32 byte accounts
			MultiLocation {
				parents: 1,
				interior: X2(Parachain(para_id), AccountId32 { id, .. }),
			} => ForeignChainAliasAccount::<AccountId>::from_para_32(para_id, id, 1),

			// Used on para-chain for sending from the relay chain
			MultiLocation { parents: 1, interior: X1(AccountId32 { id, .. }) } =>
				ForeignChainAliasAccount::<AccountId>::from_relay_32(id, 1),

			// No other conversions provided
			_ => return Err(()),
		};

		Ok(entropy.into())
	}

	fn reverse_ref(_: impl Borrow<AccountId>) -> Result<MultiLocation, ()> {
		Err(())
	}
}

impl<AccountId> ForeignChainAliasAccount<AccountId> {
	fn from_para_32(para_id: &u32, id: &[u8; 32], parents: u8) -> [u8; 32] {
		(FOREIGN_CHAIN_PREFIX_PARA_32, para_id, id, parents).using_encoded(blake2_256)
	}

	fn from_relay_32(id: &[u8; 32], parents: u8) -> [u8; 32] {
		(FOREIGN_CHAIN_PREFIX_RELAY, id, parents).using_encoded(blake2_256)
	}
}

// TODO: Is this vulnerable to DoS? It's how the instructions work
pub struct AllowNoteUnlockables;
impl ShouldExecute for AllowNoteUnlockables {
	fn should_execute<RuntimeCall>(
		_origin: &MultiLocation,
		instructions: &mut [Instruction<RuntimeCall>],
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ProcessMessageError> {
		ensure!(instructions.len() == 1, ProcessMessageError::BadFormat);
		match instructions.first() {
			Some(NoteUnlockable { .. }) => Ok(()),
			_ => Err(ProcessMessageError::BadFormat),
		}
	}
}

pub struct AllowUnlocks;
impl ShouldExecute for AllowUnlocks {
	fn should_execute<RuntimeCall>(
		_origin: &MultiLocation,
		instructions: &mut [Instruction<RuntimeCall>],
		_max_weight: Weight,
		_weight_credit: &mut Weight,
	) -> Result<(), ProcessMessageError> {
		ensure!(instructions.len() == 1, ProcessMessageError::BadFormat);
		match instructions.first() {
			Some(UnlockAsset { .. }) => Ok(()),
			_ => Err(ProcessMessageError::BadFormat),
		}
	}
}
