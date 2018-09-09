extern crate bincode;
extern crate bulletproofs;
extern crate curve25519_dalek;
extern crate libc;
extern crate rand;
#[macro_use]
extern crate arrayref;

use curve25519_dalek::{ristretto::RistrettoPoint, scalar::Scalar};
use libc::{size_t, uint64_t, uint8_t};
use rand::{OsRng, Rng};
use std::slice;

use bulletproofs::{Generators, PedersenGenerators, RangeProof, Transcript};

#[no_mangle]
pub extern "C" fn generate_ristretto_random(buf: *mut uint8_t, len: size_t) {
    let buffer = unsafe {
        assert!(!buf.is_null());
        slice::from_raw_parts_mut(buf, len as usize)
    };
    let mut rng = OsRng::new().unwrap();

    let point = RistrettoPoint::random(&mut rng);

    let point_bytes = point.compress().to_bytes();

    buffer.copy_from_slice(&point_bytes);
}

pub extern "C" fn generate_ristretto_range_proof(
    value_0: uint64_t,
    value_1: uint64_t,
    blind_0_buf: *const uint8_t,
    blind_0_buf_len: size_t,
    blind_1_buf: *const uint8_t,
    blind_1_buf_len: size_t,
    proof_buf: *mut uint8_t,
    proof_buf_len: size_t,
    value_comm_0_buf:*mut uint8_t,
    value_comm_0_buf_len:*mut uint8_t,
    value_comm_1_buf:*mut uint8_t,
    value_comm_1_buf_len:*mut uint8_t,
) {


    // Both prover and verifier have access to the generators and the proof
    let generators = Generators::new(PedersenGenerators::default(), 2, 2);

    let values: Vec<u64> = vec![value_0 as u64, value_1 as u64];

    let blind_0_buffer = c_buf_to_32_bytes_array(blind_0_buf,blind_0_buf_len);
    let blind_1_buffer = c_buf_to_32_bytes_array(blind_1_buf,blind_1_buf_len);

    let proof_buffer = unsafe {
        assert!(!proof_buf.is_null());
        slice::from_raw_parts_mut(proof_buf, proof_buf_len as usize)
    };


    let value_comm_0_buffer = unsafe {
        assert!(!value_comm_0_buf.is_null());
        slice::from_raw_parts_mut(value_comm_0_buf, value_comm_0_buf_len as usize)
    };

  let value_comm_1_buffer = unsafe {
        assert!(!value_comm_1_buf.is_null());
        slice::from_raw_parts_mut(value_comm_1_buf, value_comm_1_buf_len as usize)
    };


    let mut rng = OsRng::new().unwrap();
    let mut transcript = Transcript::new(b"AggregatedRangeProofTest");

    let (min, max) = (0u64, ((1u128 << n) - 1) as u64);
    let blindings: Vec<Scalar> = vec![Scalar::from_canonical_bytes(blind_0_buffer).unwrap(),Scalar::from_canonical_bytes(blind_1_buffer).unwrap()];

    let proof = RangeProof::prove_multiple(
        &generators,
        &mut transcript,
        &mut rng,
        &values,
        &blindings,
        n,
    ).unwrap();

    // 2. Serialize
    let proof_bytes = bincode::serialize(&proof).unwrap();

    proof_buffer.copy_from_slice(proof_bytes.as_slice());

    let pg = &generators.pedersen_gens;

    // XXX would be nice to have some convenience API for this
    let value_commitments: Vec<RistrettoPoint> = values
        .iter()
        .zip(blindings.iter())
        .map(|(&v, &v_blinding)| pg.commit(Scalar::from(v), v_blinding))
        .collect();

    value_comm_0_buffer.copy_from_slice(&value_commitments[0].compress().to_bytes());
    value_comm_1_buffer.copy_from_slice(&value_commitments[1].compress().to_bytes());
}


fn c_buf_to_32_bytes_array(buf: *const uint8_t,len: size_t)->[u8;32]{
    let buffer = unsafe {
        assert!(!buf.is_null());
        slice::from_raw_parts(buf, len as usize)
    };
    array_ref![buffer,0,32].clone()
}