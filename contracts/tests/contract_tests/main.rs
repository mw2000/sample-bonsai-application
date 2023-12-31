// Copyright 2023 RISC Zero, Inc.
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

//! Tests for the HelloBonsai contract using a mock for the Bonsai proxy contract.

pub mod utils;

use std::collections::HashMap;
use std::error::Error;

use ethers::prelude::*;
use hello_bonsai_contracts::HelloBonsai;
use hello_bonsai_methods::{MINT_ELF, MINT_ID};
use risc0_zkvm::sha::Digest;

use crate::utils::bonsai_test;

#[tokio::test]
pub async fn test_successful_contract_usage() -> Result<(), Box<dyn Error>> {
    let image_id = Digest::from(MINT_ID);
    let registry = HashMap::from([(image_id.into(), MINT_ELF)]);

    bonsai_test(registry, |client, bonsai_mock_address| async move {
        // Deploy the HelloBonsai contract.
        let hello_bonsai =
            HelloBonsai::deploy(client.clone(), (bonsai_mock_address, H256(image_id.into())))?
                .send()
                .await?;

        // Subscribe to events on HelloBonsai.
        let events = hello_bonsai.events();
        let mut subscription = events.subscribe().await?;

        // Call a function which offloads work to Bonsai.
        hello_bonsai
            .mint(U256::from(10))
            .send()
            .await?;

        // let s = "<svg height='100' width='100'><circle cx='50' cy='50' r='10' stroke='black' stroke-width='3' fill='red' /></svg>";

        // Wait for the callback to come from Bonsai.
        // let callback_log = subscription.next().await.unwrap()?;
        // assert_eq!(callback_log.n, U256::from(10));
        // assert_eq!(callback_log.svg, Bytes::from(s));

        // Check that the expected changes took place on the contract.
        // let result: String = hello_bonsai.tokenURI(U256::from(10)).call().await?;
        // assert_eq!(result, s);
        Ok(())
    })
    .await
}
