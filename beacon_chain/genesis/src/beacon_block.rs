use bls::{Signature, BLS_AGG_SIG_BYTE_SIZE};
use spec::ChainSpec;
use ssz::{encode::encode_length, Decodable, LENGTH_BYTES};
use types::{BeaconBlock, BeaconBlockBody, Hash256};

/// Generate a genesis BeaconBlock.
pub fn genesis_beacon_block(state_root: Hash256, spec: &ChainSpec) -> BeaconBlock {
    BeaconBlock {
        slot: spec.initial_slot_number,
        parent_root: spec.zero_hash,
        state_root,
        randao_reveal: spec.zero_hash,
        candidate_pow_receipt_root: spec.zero_hash,
        signature: genesis_signature(),
        body: BeaconBlockBody {
            proposer_slashings: vec![],
            casper_slashings: vec![],
            attestations: vec![],
            deposits: vec![],
            exits: vec![],
        },
    }
}

fn genesis_signature() -> Signature {
    let mut bytes = encode_length(BLS_AGG_SIG_BYTE_SIZE, LENGTH_BYTES);
    bytes.append(&mut vec![0; BLS_AGG_SIG_BYTE_SIZE]);
    let (signature, _) = match Signature::ssz_decode(&bytes, 0) {
        Ok(sig) => sig,
        Err(_) => unreachable!(),
    };
    signature
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: enhance these tests.
    // https://github.com/sigp/lighthouse/issues/117

    #[test]
    fn test_genesis() {
        let spec = ChainSpec::foundation();
        let state_root = Hash256::from("cats".as_bytes());

        // This only checks that the function runs without panic.
        genesis_beacon_block(state_root, &spec);
    }

    // Tests parent_root, randao_reveal, deposit_root are the zero hash after creation and slot == 0
    #[test]
    fn test_zero_items() {
        let spec = ChainSpec::foundation();

        // Note: state_root will not be available without a state (test in beacon_state)
        let state_root = Hash256::zero();

        let genesis_block = genesis_beacon_block(state_root, &spec);

        assert!(genesis_block.parent_root.is_zero());
        assert!(genesis_block.randao_reveal.is_zero());
        assert!(genesis_block.slot == 0);
        // assert!(genesis_block.depsoit_root.is_zero());

    }

    #[test]
    fn test_beacon_body() {
        let spec = ChainSpec::foundation();

        // Note: state_root will not be available without a state (test in beacon_state)
        let state_root = Hash256::zero();

        let genesis_block = genesis_beacon_block(state_root, &spec);

        assert!(genesis_block.body.proposer_slashings.is_empty());
        assert!(genesis_block.body.casper_slashings.is_empty());
        assert!(genesis_block.body.attestations.is_empty());
        assert!(genesis_block.body.deposits.is_empty());
        assert!(genesis_block.body.exits.is_empty());
        // Specs have changed to include 3 more variables in BeaconBody to be added later
    }

    #[test]
    fn test_signature() {
        let spec = ChainSpec::foundation();

        // Note: state_root will not be available without a state (test in beacon_state)
        let state_root = Hash256::zero();

        let genesis_block = genesis_beacon_block(state_root, &spec);

        // Signature should consist of [bytes48(0), bytes48(0)]
        // Note this is implemented using Apache Milagro BLS which requires one extra byte -> 97bytes
        let raw_sig = genesis_block.signature.as_raw();
        let raw_sig_bytes = raw_sig.as_bytes();

        assert!(raw_sig_bytes.len() == 97);
        for item in raw_sig_bytes.iter() {
            assert!(*item == 0);
        }    
    }
}