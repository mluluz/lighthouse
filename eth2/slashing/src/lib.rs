extern crate bls;
extern crate ssz;
extern crate types;

pub mod verify_slashable_attestation;

pub use crate::verify_slashable_attestation::verify_slashable_attestation;