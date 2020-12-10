use std::mem::transmute;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn now() -> u128 {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs() as u128 * 1000 + duration.subsec_millis() as u128
}

pub fn u64_to_bytes(i: u64) -> [u8;8] {
    unsafe { transmute(i.to_be()) }
}


pub fn u128_to_bytes(i: u128) -> [u8;16] {
    unsafe { transmute(i.to_be()) }
}
