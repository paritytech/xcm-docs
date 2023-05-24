// Copyright 2021 Parity Technologies (UK) Ltd.
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

pub mod parachain;
pub mod relay_chain;

use frame_support::{assert_ok, sp_tracing, traits::GenesisBuild};
use xcm::prelude::*;
use xcm_executor::traits::Convert;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

// Accounts
pub const ADMIN: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);
pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([1u8; 32]);
pub const BOB: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([2u8; 32]);

// Balances
pub type Balance = u128;
pub const UNITS: Balance = 10_000_000_000;
pub const CENTS: Balance = UNITS / 100; // 100_000_000
pub const INITIAL_BALANCE: u128 = 1 * UNITS;

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
		new_ext = para_ext(3),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		XcmConfig = relay_chain::XcmConfig,
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

pub fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	use parachain::{MsgQueue, Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, INITIAL_BALANCE), (relay_sovereign_account_id(), INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	pallet_assets::GenesisConfig::<Runtime> {
		assets: vec![
			(1u128, ADMIN, false, 1u128), // Create derivative asset for relay's native token
		],
		metadata: Default::default(),
		accounts: vec![(1u128, ALICE, INITIAL_BALANCE)],
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
