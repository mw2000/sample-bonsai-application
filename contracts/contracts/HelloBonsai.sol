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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.16;

import {IBonsaiProxy} from "./IBonsaiProxy.sol";
import {BonsaiApp} from "./BonsaiApp.sol";
import {ERC721} from "./lib/openzeppelin-contracts/contracts/token/ERC721/ERC721.sol";

/// @title A starter application using Bonsai through the on-chain proxy.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
//       or difficult to implement function to a RISC Zero guest running on Bonsai.
contract HelloBonsai is BonsaiApp, ERC721 {
    // Cache of the results calculated by our guest program in Bonsai.
    mapping(uint256 => bytes) public tokenURIOutput;

    // Initialize the contract, binding it to a specified Bonsai proxy and RISC Zero guest image.
    constructor(
        IBonsaiProxy _bonsai_proxy,
        bytes32 _image_id
    ) BonsaiApp(_bonsai_proxy, _image_id) ERC721("TEST", "TEST") {} // solhint-disable-line no-empty-blocks

    event MintingCallback(uint256 indexed n, bytes svg);

    /// @notice Returns nth number in the Fibonacci sequence.
    /// @dev The sequence is defined as 1, 1, 2, 3, 5 ... with fibonnacci(0) == 1.
    ///      Only precomputed results can be returned. Call calculate_fibonacci(n) to precompute.
    function tokenURI(uint256 tokenId) public override view returns (string memory) {
        bytes memory result = tokenURIOutput[tokenId];
        return string(result);
    }

    function mint(uint256 n) external returns (bool) {
        _safeMint(msg.sender, n, "");
        submit_bonsai_request(abi.encode(n));
        return true;
    }

    /// @notice Callback function logic for processing verified journals from Bonsai.
    function bonsai_callback(bytes memory journal) internal override {
        (uint n, bytes memory svg) = abi.decode(journal, (uint256, bytes));

        emit MintingCallback(n, svg);
        tokenURIOutput[n] = svg;
    }
}
