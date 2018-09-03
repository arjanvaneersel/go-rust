extern crate libc;
extern crate curve25519_dalek;
extern crate rand;



use libc::{uint8_t, size_t};
use std::slice;
use curve25519_dalek::ristretto::{RistrettoPoint};
use rand::{OsRng};

#[no_mangle]
pub extern "C" fn generate(buf: *mut uint8_t, len: size_t) {
    let buffer = unsafe {
        assert!(!buf.is_null());
        slice::from_raw_parts_mut(buf, len as usize)
    };
    let mut rng = OsRng::new().unwrap();

    let point = RistrettoPoint::random(&mut rng);

    let point_bytes = point.compress().to_bytes();

    buffer.copy_from_slice(&point_bytes);

}
