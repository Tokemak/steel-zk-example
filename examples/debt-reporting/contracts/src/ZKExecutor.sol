// Copyright 2025 RISC Zero, Inc.
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
// not certain on what version of solidity to use here
pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "lib/risc0-ethereum/contracts/src/IRiscZeroSetVerifier.sol";
import {Steel} from "contracts/src/Steel.sol"; // not sure where to import this
import {IZKExecutor} from "./IZKExecutor.sol";
import {TransientSlot} from "lib/openzeppelin-contracts/contracts/utils/TransientSlot.sol";
// might not be used
import {SlotDerivation} from "lib/openzeppelin-contracts/contracts/utils/SlotDerivation.sol";

// cargo build hangs (as of jan 8) not certian what this does
// import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.


 
contract ZKExecutor is IZKExecutor {

    using StorageSlot for bytes32;
    using SlotDerivation for bytes32;

    // /// @notice Image ID of the only zkVM binary to accept verification from.
    // bytes32 public constant imageID = ImageID.BALANCE_OF_ID;

    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;

     /// @notice The Auto Finance System Registry
    address public immutable systemRegistry;
    /// @notice Will revert if any blocks used for prices are more than this many seconds ago
    /// maybe make modifyable

    // no firm opinions on what these should be 
    uint256 public immutable SAFE_PRICE_MAX_LATENCY = 60 * 2; // 2 minutes
    uint256 public immutable SPOT_PRICE_MAX_LATENCY = 60 * 10; // 10 minutes
    string private constant _NAMESPACE = "AutoFinanceZKDebtReportingV1";  
 
    constructor(IRiscZeroVerifier _verifier, address _systemRegistry) {
        verifier = _verifier;
        systemRegistry = _systemRegistry;

        // I think that this should deploy a zkRootPrice Oracle Contract here
        // the zk price oracle should check
        // have prices been stored in transient storage?
        // Can someone else, eg an attacker? write to transient storage before us
        // then do debt reporting with those bad values
        // need to think about and understand attack vectors
        // the zk root price oracle will need to check that the proof is right, maybe not sure
        // get it working then set tests to check the attack vectors

    }


    function _validateProof(DestinationsZKPricesJournalCommitment calldata journal) private {
        // custom error messages?
        require(Steel.validateCommitment(journal.commitment), "Invalid commitment");
        // Verify the proof
        bytes32 journalHash = sha256(journalData);
        verifier.verify(seal, imageID, journalHash);
        require(journal.systemRegistry == systemRegistry, "Invalid SystemRegistry used by proof");
        // TODO double check the direction of this math
        require(journal.safePriceTimestamp  >= block.timestamp - SAFE_PRICE_MAX_LATENCY, "Safe price not within SAFE_PRICE_MAX_LATENCY second of current block ");
        require(journal.oldestSpotPriceTimestamp >= block.timestamp - SPOT_PRICE_MAX_LATENCY, "Oldest Spot price not within SAFE_PRICE_MAX_LATENCY second of current block ");
    }

    // not sure what the seal is
     function runDebtReporting(bytes calldata journalData, bytes calldata seal) external {
        DestinationsZKPricesJournalCommitment memory journal = abi.decode(journalData, (DestinationsZKPricesJournalCommitment));
        _validateProof(journal);
         

         _loadPriceInfoIntoTransientStorage(journal.priceInfo);

         // maybe set zk root price oracle (in storage? in memory)
         // gaurded so that only this contract can write to it
         
         // run debt reporting for the autopools, / destinations that need it
         _removePriceInfoFromTransientStorage(journal.priceInfo);

    } 

    // copied from SlotDerivation
    function setValueInNamespace(address key, address newValue) internal {
        _NAMESPACE.erc7201Slot().deriveMapping(key).getAddressSlot().value = newValue;
     }

     function getValueInNamespace(uint256 key) internal view returns (address) {
       return _NAMESPACE.erc7201Slot().deriveMapping(key).getAddressSlot().value;
     }

    function _loadPriceInfoIntoTransientStorage(PriceInfo[] priceInfo) private {

        // store it packed
        for (uint256 i = 0; i < priceInfo.length; i++) {
            
        // need to go Address -> slot
        // take that memory[slot] = packedPrices
    }


        
        


    }

    function _removePriceInfoFromTransientStorage(PriceInfo[] priceInfo) private {


    }


}
