// Copyright 2024, 2025 RISC Zero, Inc.
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

use risc0_zkvm::guest::env;
use zeth_core::db::memory::MemoryDB;
use zeth_core::stateless::client::StatelessClient;
use zeth_core_ethereum::{RethCoreDriver, RethStatelessClient};

#[no_mangle]
pub extern "C" fn __ctzsi2(x: u32) -> usize {
    x.trailing_zeros() as usize
}

// NOTE guest code
fn main() {
    let stateless_client_data_rkyv = env::read_frame();
    let stateless_client_data_pot = env::read_frame();
    env::log("====================pos 1====================");
    let hex_rkyv:String =stateless_client_data_rkyv.iter().map(|byte| format!("{:02x}",byte)).collect();
    let tmp:String = format!("stateless_client_data_rkyv: {},{}",stateless_client_data_rkyv.len(),hex_rkyv);
    env::log(&tmp);
    let hex_pot:String =stateless_client_data_pot.iter().map(|byte| format!("{:02x}",byte)).collect();
    let tmp:String = format!("stateless_client_data_pot:{},{}",stateless_client_data_pot.len(),hex_pot);
    env::log(&tmp);
    let stateless_client_data =
        <RethStatelessClient as StatelessClient<RethCoreDriver, MemoryDB>>::data_from_parts(
            &stateless_client_data_rkyv,
            &stateless_client_data_pot,
        )
        .expect("Failed to load client data from stdin");
    env::log("====================pos 2====================");
    let validation_depth = stateless_client_data.blocks.len() as u64;
    assert!(
        stateless_client_data.chain.is_ethereum(),
        "This program only supports Ethereum chains"
    );
    let chain_id = stateless_client_data.chain as u64;
    // Build the block
    env::log("Validating blocks");
    let engine = <RethStatelessClient as StatelessClient<RethCoreDriver, MemoryDB>>::validate(
        stateless_client_data,
    )
    .expect("block validation failed");
    // Build the journal (todo: make this a strategy)
    let block_hash = engine.data.parent_header.hash_slow();
    let total_difficulty = engine.data.total_difficulty;
    let journal = [
        chain_id.to_be_bytes().as_slice(),
        block_hash.0.as_slice(),
        total_difficulty.to_be_bytes::<32>().as_slice(),
        validation_depth.to_be_bytes().as_slice(),
    ]
    .concat();
    env::commit_slice(&journal.as_slice())
}
