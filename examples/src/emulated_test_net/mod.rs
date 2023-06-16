pub use codec::Encode;

pub use frame_support::{
	assert_ok, instances::Instance1, pallet_prelude::Weight, sp_io, sp_tracing,
	traits::fungibles::Inspect,
};

pub use integration_tests_common::{
	constants::{
		accounts::{ALICE, BOB},
		kusama, penpal, statemine, PROOF_SIZE_THRESHOLD, REF_TIME_THRESHOLD, XCM_V3,
	},
	AccountId, Balance,
};

pub use xcm::{
	prelude::*,
	v3::{Error, NetworkId::Kusama as KusamaId},
};
pub use xcm_emulator::{
	assert_expected_events, cumulus_pallet_dmp_queue, decl_test_networks, decl_test_parachains,
	decl_test_relay_chains, Parachain, RelayChain, TestExt,
};

pub use sp_core::{sr25519, storage::Storage, Get};

use xcm_executor::traits::Convert;

decl_test_relay_chains! {
	pub struct Kusama {
		genesis = kusama::genesis(),
		on_init = (),
		runtime = {
			Runtime: kusama_runtime::Runtime,
			RuntimeOrigin: kusama_runtime::RuntimeOrigin,
			RuntimeCall: kusama_runtime::RuntimeCall,
			RuntimeEvent: kusama_runtime::RuntimeEvent,
			MessageQueue: kusama_runtime::MessageQueue,
			XcmConfig: kusama_runtime::xcm_config::XcmConfig,
			SovereignAccountOf: kusama_runtime::xcm_config::SovereignAccountOf,
			System: kusama_runtime::System,
			Balances: kusama_runtime::Balances,
		},
		pallets_extra = {
			XcmPallet: kusama_runtime::XcmPallet,
		}
	}
}

decl_test_parachains! {
	pub struct AssetHubKusama {
		genesis = statemine::genesis(),
		on_init = (),
		runtime = {
			Runtime: statemine_runtime::Runtime,
			RuntimeOrigin: statemine_runtime::RuntimeOrigin,
			RuntimeCall: statemine_runtime::RuntimeCall,
			RuntimeEvent: statemine_runtime::RuntimeEvent,
			XcmpMessageHandler: statemine_runtime::XcmpQueue,
			DmpMessageHandler: statemine_runtime::DmpQueue,
			LocationToAccountId: statemine_runtime::xcm_config::LocationToAccountId,
			System: statemine_runtime::System,
			Balances: statemine_runtime::Balances,
			ParachainSystem: statemine_runtime::ParachainSystem,
			ParachainInfo: statemine_runtime::ParachainInfo,
		},
		pallets_extra = {
			PolkadotXcm: statemine_runtime::PolkadotXcm,
			Assets: statemine_runtime::Assets,
			ForeignAssets: statemine_runtime::Assets,
		}
	},
	pub struct PenpalKusama {
		genesis = penpal::genesis(penpal::PARA_ID),
		on_init = (),
		runtime = {
			Runtime: penpal_runtime::Runtime,
			RuntimeOrigin: penpal_runtime::RuntimeOrigin,
			RuntimeCall: penpal_runtime::RuntimeCall,
			RuntimeEvent: penpal_runtime::RuntimeEvent,
			XcmpMessageHandler: penpal_runtime::XcmpQueue,
			DmpMessageHandler: penpal_runtime::DmpQueue,
			LocationToAccountId: penpal_runtime::xcm_config::LocationToAccountId,
			System: penpal_runtime::System,
			Balances: penpal_runtime::Balances,
			ParachainSystem: penpal_runtime::ParachainSystem,
			ParachainInfo: penpal_runtime::ParachainInfo,
		},
		pallets_extra = {
			PolkadotXcm: penpal_runtime::PolkadotXcm,
			Assets: penpal_runtime::Assets,
		}
	}
}

decl_test_networks! {
	pub struct KusamaMockNet {
		relay_chain = Kusama,
		parachains = vec![
			AssetHubKusama,
			PenpalKusama,
		],

	}
}
