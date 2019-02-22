#![cfg(test)]

use crate::test_utils::{BeaconStateTestBuilder, SeedableRng, TestRandom, XorShiftRng};
use ssz::{ssz_encode, Decodable};
use super::*;

#[test]
pub fn can_produce_genesis_block() {
    let builder = BeaconStateTestBuilder::with_random_validators(2);

    builder.build().unwrap();
}

#[test]
pub fn test_ssz_round_trip() {
    let mut rng = XorShiftRng::from_seed([42; 16]);
    let original = BeaconState::random_for_test(&mut rng);

    let bytes = ssz_encode(&original);
    let (decoded, _) = <_>::ssz_decode(&bytes, 0).unwrap();

    assert_eq!(original, decoded);
}

#[test]
pub fn test_hash_tree_root_internal() {
    let mut rng = XorShiftRng::from_seed([42; 16]);
    let original = BeaconState::random_for_test(&mut rng);

    let result = original.hash_tree_root_internal();

    assert_eq!(result.len(), 32);
    // TODO: Add further tests
    // https://github.com/sigp/lighthouse/issues/170
}
