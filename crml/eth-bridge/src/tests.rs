/* Copyright 2019-2021 Centrality Investments Limited
*
* Licensed under the LGPL, Version 3.0 (the "License");
* you may not use this file except in compliance with the License.
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
* You may obtain a copy of the License at the root of this project source code,
* or at:
*     https://centrality.ai/licenses/gplv3.txt
*     https://centrality.ai/licenses/lgplv3.txt
*/

use crate as crml_eth_bridge;
use crate::{Config, Module};
use cennznet_primitives::eth::crypto::AuthorityId;
use crml_support::{EventClaimSubscriber, FinalSessionTracker, NotarizationRewardHandler, H160, H256 as H256Crml};
use frame_support::traits::OneSessionHandler;
use frame_support::{
	parameter_types,
	storage::StorageValue,
	traits::{UnixTime, ValidatorSet as ValidatorSetT},
};
use sp_core::{
	ecdsa::Signature,
	offchain::{testing, OffchainDbExt, OffchainWorkerExt},
	Public, H256,
};
use sp_runtime::offchain::StorageKind;
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, Convert, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
	Percent,
};

type SessionIndex = u32;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

type Extrinsic = TestXt<Call, ()>;
type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		EthBridge: crml_eth_bridge::{Pallet, Call, Storage, Event, ValidateUnsigned},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}
impl frame_system::Config for TestRuntime {
	type BlockWeights = ();
	type BlockLength = ();
	type BaseCallFilter = frame_support::traits::Everything;
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = Call;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type BlockHashCount = BlockHashCount;
	type Event = Event;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
}

parameter_types! {
	pub const DefaultListingDuration: u64 = 5;
	pub const MaxAttributeLength: u8 = 140;
	pub const NotarizationThreshold: Percent = Percent::from_parts(66_u8);
}
impl Config for TestRuntime {
	type AuthoritySet = MockValidatorSet;
	type FinalSessionTracker = MockFinalSessionTracker;
	type EthyId = AuthorityId;
	type NotarizationThreshold = NotarizationThreshold;
	type RewardHandler = MockRewardHandler;
	type Subscribers = MockClaimSubscriber;
	type UnixTime = MockUnixTime;
	type Call = Call;
	type Event = Event;
}

pub struct NoopConverter<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> Convert<T::AccountId, Option<T::AccountId>> for NoopConverter<T> {
	fn convert(address: T::AccountId) -> Option<T::AccountId> {
		Some(address)
	}
}

pub struct MockValidatorSet;
impl ValidatorSetT<AccountId> for MockValidatorSet {
	type ValidatorId = AccountId;
	type ValidatorIdOf = NoopConverter<TestRuntime>;
	/// Returns current session index.
	fn session_index() -> SessionIndex {
		1
	}
	/// Returns the active set of validators.
	fn validators() -> Vec<Self::ValidatorId> {
		Default::default()
	}
}

pub struct MockClaimSubscriber;
impl EventClaimSubscriber for MockClaimSubscriber {
	/// Notify subscriber about a successful event claim for the given event data
	fn on_success(_event_claim_id: u64, _contract_address: &H160, _event_signature: &H256Crml, _event_data: &[u8]) {}
	/// Notify subscriber about a failed event claim for the given event data
	fn on_failure(_event_claim_id: u64, _contract_address: &H160, _event_signature: &H256Crml, _event_data: &[u8]) {}
}

/// Mock final session tracker
pub struct MockFinalSessionTracker;
impl FinalSessionTracker for MockFinalSessionTracker {
	fn is_next_session_final() -> (bool, bool) {
		// at block 1, next session is final
		(frame_system::Pallet::<TestRuntime>::block_number() == 1, false)
	}
	fn is_active_session_final() -> bool {
		// at block 2, the active session is final
		frame_system::Pallet::<TestRuntime>::block_number() == 2
	}
}

/// Returns the current system time
pub struct MockRewardHandler;
impl NotarizationRewardHandler for MockRewardHandler {
	type AccountId = AccountId;
	fn reward_notary(_notary: &Self::AccountId) {
		// Do nothing
	}
}

/// Returns the current system time
pub struct MockUnixTime;
impl UnixTime for MockUnixTime {
	fn now() -> core::time::Duration {
		std::time::SystemTime::now()
			.duration_since(std::time::UNIX_EPOCH)
			.unwrap()
	}
}

impl frame_system::offchain::SigningTypes for TestRuntime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for TestRuntime
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for TestRuntime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

#[derive(Clone, Copy, Default)]
pub struct ExtBuilder {
	next_session_final: bool,
	active_session_final: bool,
}

impl ExtBuilder {
	pub fn active_session_final(&mut self) -> &mut Self {
		self.active_session_final = true;
		self
	}
	pub fn next_session_final(&mut self) -> &mut Self {
		self.next_session_final = true;
		self
	}
	pub fn build(self) -> sp_io::TestExternalities {
		let mut ext: sp_io::TestExternalities = frame_system::GenesisConfig::default()
			.build_storage::<TestRuntime>()
			.unwrap()
			.into();
		if self.next_session_final {
			ext.execute_with(|| frame_system::Pallet::<TestRuntime>::set_block_number(1));
		} else if self.active_session_final {
			ext.execute_with(|| frame_system::Pallet::<TestRuntime>::set_block_number(2));
		}

		ext
	}
}

/// Mock eth-http endpoint
const MOCK_ETH_HTTP_URI: [u8; 31] = *b"http://ethereum-rpc.example.com";

/// Test request
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
struct TestRequest {
	message: String,
}

#[test]
fn pre_last_session_change() {
	ExtBuilder::default().next_session_final().build().execute_with(|| {
		let current_keys = vec![
			AuthorityId::from_slice(&[1_u8; 33]),
			AuthorityId::from_slice(&[2_u8; 33]),
		];
		let next_keys = vec![
			AuthorityId::from_slice(&[3_u8; 33]),
			AuthorityId::from_slice(&[4_u8; 33]),
		];
		let event_proof_id = Module::<TestRuntime>::next_proof_id();

		Module::<TestRuntime>::handle_authorities_change(current_keys, next_keys.clone());

		assert_eq!(Module::<TestRuntime>::next_notary_keys(), next_keys);
		assert_eq!(Module::<TestRuntime>::notary_set_proof_id(), event_proof_id);
		assert_eq!(Module::<TestRuntime>::next_proof_id(), event_proof_id + 1);
	});
}

#[test]
fn last_session_change() {
	ExtBuilder::default().active_session_final().build().execute_with(|| {
		let current_set_id = Module::<TestRuntime>::notary_set_id();

		// setup storage
		let current_keys = vec![
			AuthorityId::from_slice(&[1_u8; 33]),
			AuthorityId::from_slice(&[2_u8; 33]),
		];
		crate::NotaryKeys::<TestRuntime>::put(&current_keys);
		let next_keys = vec![
			AuthorityId::from_slice(&[3_u8; 33]),
			AuthorityId::from_slice(&[4_u8; 33]),
		];
		crate::NextNotaryKeys::<TestRuntime>::put(&next_keys);

		// current session is last in era: starting
		Module::<TestRuntime>::handle_authorities_change(current_keys, next_keys.clone());
		assert!(Module::<TestRuntime>::bridge_paused());
		// current session is last in era: finishing
		<Module<TestRuntime> as OneSessionHandler<AccountId>>::on_before_session_ending();
		assert_eq!(Module::<TestRuntime>::notary_keys(), next_keys);
		assert_eq!(Module::<TestRuntime>::notary_set_id(), current_set_id + 1);
		assert!(!Module::<TestRuntime>::bridge_paused());
	});
}

#[test]
fn eth_client_http_request() {
	let (offchain, offchain_state) = testing::TestOffchainExt::new();
	let mut t = sp_io::TestExternalities::default();
	t.register_extension(OffchainDbExt::new(offchain.clone()));
	t.register_extension(OffchainWorkerExt::new(offchain));
	// Set the ethereum http endpoint for OCW queries
	t.execute_with(|| sp_io::offchain::local_storage_set(StorageKind::PERSISTENT, b"ETH_HTTP", &MOCK_ETH_HTTP_URI));

	// Setup
	// Mock an ethereum JSON-RPC response
	let request_body = TestRequest {
		message: "hello ethereum".to_string(),
	};
	let request_body_raw = serde_json::to_string(&request_body).unwrap();
	{
		let mut offchain_state = offchain_state.write();
		offchain_state.expect_request(testing::PendingRequest {
			method: "POST".into(),
			uri: core::str::from_utf8(&MOCK_ETH_HTTP_URI)
				.expect("valid utf8")
				.to_string(),
			body: request_body_raw.as_bytes().to_vec(),
			response: Some(br#"{"message":"hello cennznet"}"#.to_vec()),
			headers: vec![
				("Content-Type".to_string(), "application/json".to_string()),
				("Content-Length".to_string(), request_body_raw.len().to_string()),
			],
			sent: true,
			..Default::default()
		});
	}

	// Test
	t.execute_with(|| {
		let response = Module::<TestRuntime>::query_eth_client(request_body).expect("got response");
		assert_eq!(
			serde_json::from_slice::<'_, TestRequest>(response.as_slice()).unwrap(),
			TestRequest {
				message: "hello cennznet".to_string()
			}
		);
	})
}

#[test]
fn prunes_expired_events() {}

#[test]
fn double_claim_fails() {}

#[test]
fn invalid_notarization_fails() {}
