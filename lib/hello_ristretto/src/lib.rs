extern crate libc;
extern crate curve25519_dalek;
extern crate rand;
extern crate bulletproofs;
extern crate bincode;


use libc::{uint8_t,uint64_t,size_t};
use std::slice;
use curve25519_dalek::{
    ristretto::RistrettoPoint,
    scalar::Scalar
    };
use rand::{Rng,OsRng};

use bulletproofs::{Transcript, RangeProof, PedersenGenerators,Generators};


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

pub extern "C" fn generate_ristretto_range_proof(proof_buf: *mut uint8_t, proof_buf_len:size_t){

        let n:usize = 1;
        let m:usize = 1;


        // Both prover and verifier have access to the generators and the proof
        let generators = Generators::new(PedersenGenerators::default(), n, m);


            let buffer = unsafe {
                assert!(!proof_buf.is_null());
                slice::from_raw_parts_mut(proof_buf, proof_buf_len as usize)
            };

            let mut rng = OsRng::new().unwrap();
            let mut transcript = Transcript::new(b"AggregatedRangeProofTest");

            let (min, max) = (0u64, ((1u128 << n) - 1) as u64);
            let values: Vec<u64> = (0..m).map(|_| rng.gen_range(min, max)).collect();
            let blindings: Vec<Scalar> = (0..m).map(|_| Scalar::random(&mut rng)).collect();

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

            let pg = &generators.pedersen_gens;

            // XXX would be nice to have some convenience API for this
            let value_commitments: Vec<RistrettoPoint> = values
                .iter()
                .zip(blindings.iter())
                .map(|(&v, &v_blinding)| pg.commit(Scalar::from(v), v_blinding))
                .collect();
        }
