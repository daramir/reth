//! Ethereum protocol-related constants

use crate::{H256, U256};
use hex_literal::hex;

/// The first four bytes of the call data for a function call specifies the function to be called.
pub const SELECTOR_LEN: usize = 4;

/// The minimal value the basefee can decrease to.
///
/// The `BASE_FEE_MAX_CHANGE_DENOMINATOR` <https://eips.ethereum.org/EIPS/eip-1559> is `8`, or 12.5%.
/// Once the base fee has dropped to `7` WEI it cannot decrease further because 12.5% of 7 is less
/// than 1.
pub const MIN_PROTOCOL_BASE_FEE: u128 = 7;

/// Same as [MIN_PROTOCOL_BASE_FEE] but as a U256.
pub const MIN_PROTOCOL_BASE_FEE_U256: U256 = U256::from_limbs([7u64, 0, 0, 0]);

/// Initial base fee as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
pub const EIP1559_INITIAL_BASE_FEE: u64 = 1_000_000_000;

/// Base fee max change denominator as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
pub const EIP1559_BASE_FEE_MAX_CHANGE_DENOMINATOR: u64 = 8;

/// Elasticity multiplier as defined in [EIP-1559](https://eips.ethereum.org/EIPS/eip-1559)
pub const EIP1559_ELASTICITY_MULTIPLIER: u64 = 2;

/// Multiplier for converting gwei to wei.
pub const GWEI_TO_WEI: u64 = 1_000_000_000;

/// Multiplier for converting finney (milliether) to wei.
pub const FINNEY_TO_WEI: u128 = (GWEI_TO_WEI as u128) * 1_000_000;

/// Multiplier for converting ether to wei.
pub const ETH_TO_WEI: u128 = FINNEY_TO_WEI * 1000;

/// The Ethereum mainnet genesis hash.
pub const MAINNET_GENESIS: H256 =
    H256(hex!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3"));

/// Goerli genesis hash.
pub const GOERLI_GENESIS: H256 =
    H256(hex!("bf7e331f7f7c1dd2e05159666b3bf8bc7a8a3a9eb1d518969eab529dd9b88c1a"));

/// Sepolia genesis hash.
pub const SEPOLIA_GENESIS: H256 =
    H256(hex!("25a5cc106eea7138acab33231d7160d69cb777ee0c2c553fcddf5138993e6dd9"));

/// Keccak256 over empty array.
pub const KECCAK_EMPTY: H256 =
    H256(hex!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"));

/// Ommer root of empty list.
pub const EMPTY_OMMER_ROOT: H256 =
    H256(hex!("1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"));

/// hash of an empty set `keccak256(rlp([]))`
const EMPTY_SET_HASH: H256 =
    H256(hex!("56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"));

/// Transactions root of empty receipts set.
pub const EMPTY_RECEIPTS: H256 = EMPTY_SET_HASH;

/// Transactions root of empty transactions set.
pub const EMPTY_TRANSACTIONS: H256 = EMPTY_SET_HASH;

/// Withdrawals root of empty withdrawals set.
pub const EMPTY_WITHDRAWALS: H256 = EMPTY_SET_HASH;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_protocol_sanity() {
        assert_eq!(MIN_PROTOCOL_BASE_FEE_U256.to::<u128>(), MIN_PROTOCOL_BASE_FEE);
    }
}
