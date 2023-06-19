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

#![no_main]
#![no_std]
use ethabi::ethereum_types::U256;
use ethabi::Bytes;
use ethabi::{ParamType, Token};
use risc0_zkvm::guest::env;

#[macro_use]
extern crate alloc;

risc0_zkvm::guest::entry!(main);

// fn factorize(n: U256) -> (U256, U256) {
//     let mut divisor = 2;
//     while n.as_u32() % divisor != 0 {
//         divisor += 1;
//     }
//     let other_factor = n.as_u32() / divisor;
//     return (U256::from(divisor), U256::from(other_factor));
// }

fn mint(n: U256) -> Bytes {
    let text = format!("<svg height='100' width='100'><circle cx='50' cy='50' r='{}' stroke='black' stroke-width='3' fill='red' /></svg>", n);
    return Bytes::from(text);
}

const INPUT_LEN: usize = core::mem::size_of::<U256>();

pub fn main() {
    // NOTE: env::read_slice requires a length argument. Reads must be of known
    // length. https://github.com/risc0/risc0/issues/402
    let input = ethabi::decode_whole(&[ParamType::Uint(256)], env::read_slice(INPUT_LEN)).unwrap();
    let n: U256 = input[0].clone().into_uint().unwrap();

    // Run the computation.
    // let (p,q) = factorize(n);
    let minted = mint(n);

    // Commit the journal that will be decoded in the application contract.
    env::commit_slice(&ethabi::encode(&[Token::Uint(n), Token::Bytes(minted)]));
}
