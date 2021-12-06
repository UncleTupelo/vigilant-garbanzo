// Copyright 2020 Parity Technologies (UK) Ltd.
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

//! End to end runtime tests

use substrate_test_runner::{Node, ChainInfo, SignatureVerificationOverride, base_path};
use grandpa::GrandpaBlockImport;
use sc_service::{
    TFullBackend, TFullClient, Configuration, TaskManager, new_full_parts, TaskExecutor,
    BasePath, Role, DatabaseConfig, KeepBlocks, TransactionStorageMode, ChainSpec,
};
use sp_runtime::generic::Era;
use std::sync::Arc;
use sp_inherents::InherentDataProviders;
use sc_consensus_babe::BabeBlockImport;
use sp_keystore::SyncCryptoStorePtr;
use sp_keyring::sr25519::Keyring::Alice;
use polkadot_service::chain_spec::polkadot_config;
use polkadot_runtime_common::claims;
use sp_consensus_babe::AuthorityId;
use sc_consensus_manual_seal::{ConsensusDataProvider, consensus::babe::BabeConsensusDataProvider};
use sp_keyring::Sr25519Keyring;
use sc_service::config::{NetworkConfiguration, KeystoreConfig};
use sc_executor::WasmExecutionMethod;
use sc_network::{multiaddr, config::TransportConfig};
use sc_client_api::execution_extensions::ExecutionStrategies;
use sc_informant::OutputFormat;

type BlockImport<B, BE, C, SC> = BabeBlockImport<B, C, GrandpaBlockImport<BE, B, C, SC>>;

sc_executor::native_executor_instance!(
	pub Executor,
	polkadot_runtime::api::dispatch,
	polkadot_runtime::native_version,
	SignatureVerificationOverride,
);

/// ChainInfo implementation.
struct PolkadotChainInfo;

impl ChainInfo for PolkadotChainInfo {
    type Block = polkadot_primitives::v1::Block;
    type Executor = Executor;
    type Runtime = polkadot_runtime::Runtime;
    type RuntimeApi = polkadot_runtime::RuntimeApi;
    type SelectChain = sc_consensus::LongestChain<TFullBackend<Self::Block>, Self::Block>;
    type BlockImport = BlockImport<
        Self::Block,
        TFullBackend<Self::Block>,
        TFullClient<Self::Block, Self::RuntimeApi, Self::Executor>,
        Self::SelectChain,
    >;
    type SignedExtras = polkadot_runtime::SignedExtra;

    fn configuration(task_executor: TaskExecutor) -> Configuration {
        let mut chain_spec = polkadot_config().expect("failed to create chainspec");
        let base_path = if let Some(base) = base_path() {
            BasePath::new(base)
        } else {
            BasePath::new_temp_dir().expect("couldn't create a temp dir")
        };
        let root_path = base_path.path().to_path_buf().join("chains").join(chain_spec.id());

        let key_seed = Sr25519Keyring::Alice.to_seed();
        let storage = chain_spec
            .as_storage_builder()
            .build_storage()
            .expect("could not build storage");

        chain_spec.set_storage(storage);

        let mut network_config = NetworkConfiguration::new(
            format!("Test Node for: {}", key_seed),
            "network/test/0.1",
            Default::default(),
            None,
        );
        let informant_output_format = OutputFormat { enable_color: false };

        network_config.allow_non_globals_in_dht = true;

        network_config
            .listen_addresses
            .push(multiaddr::Protocol::Memory(rand::random()).into());

        network_config.transport = TransportConfig::MemoryOnly;

        Configuration {
            impl_name: "test-node".to_string(),
            impl_version: "0.1".to_string(),
            role: Role::Authority,
            task_executor,
            transaction_pool: Default::default(),
            network: network_config,
            keystore: KeystoreConfig::Path {
                path: root_path.join("key"),
                password: None,
            },
            database: DatabaseConfig::RocksDb {
                path: root_path.join("db"),
                cache_size: 128,
            },
            state_cache_size: 16777216,
            state_cache_child_ratio: None,
            chain_spec: Box::new(chain_spec),
            wasm_method: WasmExecutionMethod::Interpreted,
            // NOTE: we enforce the use of the wasm runtime to make use of the signature overrides
            execution_strategies: ExecutionStrategies {
                syncing: sc_client_api::ExecutionStrategy::NativeWhenPossible,
                importing: sc_client_api::ExecutionStrategy::NativeWhenPossible,
                block_construction: sc_client_api::ExecutionStrategy::NativeWhenPossible,
                offchain_worker: sc_client_api::ExecutionStrategy::NativeWhenPossible,
                other: sc_client_api::ExecutionStrategy::NativeWhenPossible,
            },
            rpc_http: None,
            rpc_ws: None,
            rpc_ipc: None,
            rpc_ws_max_connections: None,
            rpc_cors: None,
            rpc_methods: Default::default(),
            prometheus_config: None,
            telemetry_endpoints: None,
            telemetry_external_transport: None,
            default_heap_pages: None,
            offchain_worker: Default::default(),
            force_authoring: false,
            disable_grandpa: false,
            dev_key_seed: Some(key_seed),
            tracing_targets: None,
            tracing_receiver: Default::default(),
            max_runtime_instances: 8,
            announce_block: true,
            base_path: Some(base_path),
            wasm_runtime_overrides: None,
            informant_output_format,
            disable_log_reloading: false,
            keystore_remote: None,
            keep_blocks: KeepBlocks::All,
            state_pruning: Default::default(),
            transaction_storage: TransactionStorageMode::BlockBody,
            telemetry_handle: Default::default(),
        }
    }

    fn signed_extras(from: <Self::Runtime as frame_system::Config>::AccountId) -> Self::SignedExtras {
        (
            frame_system::CheckSpecVersion::<Self::Runtime>::new(),
            frame_system::CheckTxVersion::<Self::Runtime>::new(),
            frame_system::CheckGenesis::<Self::Runtime>::new(),
            frame_system::CheckMortality::<Self::Runtime>::from(Era::Immortal),
            frame_system::CheckNonce::<Self::Runtime>::from(frame_system::Module::<Self::Runtime>::account_nonce(from)),
            frame_system::CheckWeight::<Self::Runtime>::new(),
            pallet_transaction_payment::ChargeTransactionPayment::<Self::Runtime>::from(0),
            claims::PrevalidateAttests::<Self::Runtime>::new(),
        )
    }

    fn create_client_parts(
        config: &Configuration,
    ) -> Result<
        (
            Arc<TFullClient<Self::Block, Self::RuntimeApi, Self::Executor>>,
            Arc<TFullBackend<Self::Block>>,
            SyncCryptoStorePtr,
            TaskManager,
            InherentDataProviders,
            Option<
                Box<
                    dyn ConsensusDataProvider<
                        Self::Block,
                        Transaction = sp_api::TransactionFor<
                            TFullClient<Self::Block, Self::RuntimeApi, Self::Executor>,
                            Self::Block,
                        >,
                    >,
                >,
            >,
            Self::SelectChain,
            Self::BlockImport,
        ),
        sc_service::Error,
    > {
        let (client, backend, keystore, task_manager) =
            new_full_parts::<Self::Block, Self::RuntimeApi, Self::Executor>(config)?;
        let client = Arc::new(client);

        let inherent_providers = InherentDataProviders::new();
        let select_chain = sc_consensus::LongestChain::new(backend.clone());

        let (grandpa_block_import, ..) =
            grandpa::block_import(client.clone(), &(client.clone() as Arc<_>), select_chain.clone())?;

        let (block_import, babe_link) = sc_consensus_babe::block_import(
            sc_consensus_babe::Config::get_or_compute(&*client)?,
            grandpa_block_import,
            client.clone(),
        )?;

        let consensus_data_provider = BabeConsensusDataProvider::new(
            client.clone(),
            keystore.sync_keystore(),
            &inherent_providers,
            babe_link.epoch_changes().clone(),
            vec![(AuthorityId::from(Alice.public()), 1000)],
        )
            .expect("failed to create ConsensusDataProvider");

        Ok((
            client,
            backend,
            keystore.sync_keystore(),
            task_manager,
            inherent_providers,
            Some(Box::new(consensus_data_provider)),
            select_chain,
            block_import,
        ))
    }

    fn dispatch_with_root(_call: <Self::Runtime as frame_system::Config>::Call, _node: &mut Node<Self>) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use substrate_test_runner::Node;

    #[test]
    fn it_works() {
        let mut node = Node::<PolkadotChainInfo>::new().unwrap();
        node.seal_blocks(15_000);
    }
}
