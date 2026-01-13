// SPDX-License-Identifier: Apache-2.0
// note licenses are not consistant, need to gick one and use it

pragma solidity ^0.8.20;

library PackedGetRangePriceLPCodec {
    struct PackedComputedGetRangePriceLP {
        uint112 averageSpotPriceInQuote;
        uint112 latestSafePriceInQuote;
        uint8 isSpotSafeZK;
    }

    uint256 private constant MASK_112 = uint256(type(uint112).max);
    // todo double check this math to make sure it is bit packing right

    function unpack(PackedComputedGetRangePriceLP memory p) internal pure returns (uint256 out) {
        out = uint256(p.averageSpotPriceInQuote) | (uint256(p.latestSafePriceInQuote) << 112)
            | (uint256(p.isSpotSafeZK) << 224);
    }

    function pack(uint256 x) internal pure returns (PackedComputedGetRangePriceLP memory p) {
        p.averageSpotPriceInQuote = uint112(x & MASK_112);
        p.latestSafePriceInQuote = uint112((x >> 112) & MASK_112);
        p.isSpotSafeZK = uint8(x >> 224);
    }
}
