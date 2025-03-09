// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloy_genesis::Genesis;
use once_cell::sync::Lazy;
use reth_chainspec::{
    once_cell_set, BaseFeeParams, BaseFeeParamsKind, Chain, ChainSpec, DepositContract,
    EthereumHardfork, DEV_HARDFORKS,
};
use reth_primitives::constants::ETHEREUM_BLOCK_GAS_LIMIT;
use reth_primitives::revm_primitives::{address, b256, U256};
use reth_primitives::{
    DEV_GENESIS_HASH, HOLESKY_GENESIS_HASH, MAINNET_GENESIS_HASH, SEPOLIA_GENESIS_HASH,
};
use reth_revm::primitives::bytes;
use std::sync::Arc;

/// The Ethereum mainnet spec
pub static MAINNET: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    ChainSpec {
        chain: Chain::mainnet(),
        genesis: {
            let mut genesis = Genesis::default()
                .with_nonce(0x42)
                .with_extra_data(bytes!(
                    "11bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82fa"
                ))
                .with_gas_limit(0x1388)
                .with_difficulty(U256::from(0x400000000u128));
            genesis.config.dao_fork_support = true;
            genesis
        },
        genesis_hash: once_cell_set(MAINNET_GENESIS_HASH),
        genesis_header: Default::default(),
        // <https://etherscan.io/block/15537394>
        paris_block_and_final_difficulty: Some((
            15537394,
            U256::from(58_750_003_716_598_352_816_469u128),
        )),
        hardforks: EthereumHardfork::mainnet().into(),
        // https://etherscan.io/tx/0xe75fb554e433e03763a1560646ee22dcb74e5274b34c5ad644e7c0f619a7e1d0
        deposit_contract: Some(DepositContract::new(
            address!("00000000219ab540356cbb839cbe05303d7705fa"),
            11052984,
            b256!("649bbc62d0e31342afea4e5cd82d4049e7e1ee912fc0889aa790803be39038c5"),
        )),
        base_fee_params: BaseFeeParamsKind::Constant(BaseFeeParams::ethereum()),
        max_gas_limit: ETHEREUM_BLOCK_GAS_LIMIT,
        prune_delete_limit: 20000,
    }
    .into()
});

/// The Sepolia spec
pub static SEPOLIA: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    ChainSpec {
        chain: Chain::sepolia(),
        genesis: {
            let mut genesis = Genesis::default()
                .with_timestamp(0x6159af19)
                .with_extra_data(bytes!(
                    "5365706f6c69612c20417468656e732c204174746963612c2047726565636521"
                ))
                .with_gas_limit(0x1c9c380)
                .with_difficulty(U256::from(0x20000u128));
            genesis.config.dao_fork_support = true;
            genesis
        },
        genesis_hash: once_cell_set(SEPOLIA_GENESIS_HASH),
        genesis_header: Default::default(),
        // <https://sepolia.etherscan.io/block/1450409>
        paris_block_and_final_difficulty: Some((1450409, U256::from(17_000_018_015_853_232u128))),
        hardforks: EthereumHardfork::sepolia().into(),
        // https://sepolia.etherscan.io/tx/0x025ecbf81a2f1220da6285d1701dc89fb5a956b62562ee922e1a9efd73eb4b14
        deposit_contract: Some(DepositContract::new(
            address!("7f02c3e3c98b133055b8b348b2ac625669ed295d"),
            1273020,
            b256!("649bbc62d0e31342afea4e5cd82d4049e7e1ee912fc0889aa790803be39038c5"),
        )),
        base_fee_params: BaseFeeParamsKind::Constant(BaseFeeParams::ethereum()),
        max_gas_limit: ETHEREUM_BLOCK_GAS_LIMIT,
        prune_delete_limit: 10000,
    }
    .into()
});

/// The Holesky spec
pub static HOLESKY: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    ChainSpec {
        chain: Chain::holesky(),
        genesis: {
            let mut genesis = Genesis::default()
                .with_nonce(0x1234)
                .with_timestamp(1695902100)
                .with_extra_data(bytes!("017D7840"))
                .with_difficulty(U256::from(0x01u128));
            genesis.config.dao_fork_support = true;
            genesis
        },
        genesis_hash: once_cell_set(HOLESKY_GENESIS_HASH),
        genesis_header: Default::default(),
        paris_block_and_final_difficulty: Some((0, U256::from(1))),
        hardforks: EthereumHardfork::holesky().into(),
        deposit_contract: Some(DepositContract::new(
            address!("4242424242424242424242424242424242424242"),
            0,
            b256!("649bbc62d0e31342afea4e5cd82d4049e7e1ee912fc0889aa790803be39038c5"),
        )),
        base_fee_params: BaseFeeParamsKind::Constant(BaseFeeParams::ethereum()),
        max_gas_limit: ETHEREUM_BLOCK_GAS_LIMIT,
        prune_delete_limit: 10000,
    }
    .into()
});

/// NOTE Dev testnet specification
///
/// Includes 20 prefunded accounts with `10_000` ETH each derived from mnemonic "test test test test
/// test test test test test test test junk".
pub static DEV: Lazy<Arc<ChainSpec>> = Lazy::new(|| {
    let spec = ChainSpec {
        chain: Chain::dev(),
        genesis: {
            let mut genesis = Genesis::default()
            .with_nonce(0)
            .with_timestamp(1741521992)
            .with_difficulty(U256::from(0x01u128))
            .with_gas_limit(0x1c9c380)
            .with_extra_data(bytes!("0000000000000000000000000000000000000000000000000000000000000000d4bdb75a2a23effc556b680a7b890799867ffd900000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"));
            genesis.config.dao_fork_support=true;
            genesis.config.chain_id=Chain::dev().id();
            genesis.config.homestead_block=Some(0);
            genesis.config.eip150_block=Some(0);
            genesis.config.eip158_block=Some(0);
            genesis.config.byzantium_block=Some(0);
            genesis.config.constantinople_block=Some(0);
            genesis.config.petersburg_block=Some(0);
            genesis.config.istanbul_block=Some(0);
            genesis.config.muir_glacier_block=Some(0);
            genesis.config.berlin_block=Some(0);
            genesis.config.london_block=Some(0);
            genesis.config.arrow_glacier_block=Some(0);
            genesis.config.gray_glacier_block=Some(0);
            genesis.config.shanghai_time=Some(1740705554);
            genesis.config.cancun_time=Some(1740705554);
            genesis.config.prague_time=Some(1740705554);
            genesis.config.terminal_total_difficulty=Some(U256::from(0));
            genesis.config.terminal_total_difficulty_passed=true;
            genesis
        },
        genesis_hash: once_cell_set(b256!("4db3b9bf5f853f2b65c04b4977f3e8263ee6144d06a17a9cce0fdb36382f4a65")),
        paris_block_and_final_difficulty: Some((0, U256::from(0))),
        hardforks: DEV_HARDFORKS.clone(),
        base_fee_params: BaseFeeParamsKind::Constant(BaseFeeParams::ethereum()),
        deposit_contract: Some(DepositContract::new(
            address!("2d2d2d62322d6875622d65786563746f722d2d2d"),
            0,
            b256!("649bbc62d0e31342afea4e5cd82d4049e7e1ee912fc0889aa790803be39038c5"),
        )),
        ..Default::default()
    }
    .into();
    println!("DEV: {:?}", spec);
    spec
});
