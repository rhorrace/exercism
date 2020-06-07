extern crate rand;

use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng()
        .gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow_mod(g as u128, a as u128, p as u128)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow_mod(b_pub as u128, a as u128, p as u128)
}

fn pow_mod(mut b: u128, mut e: u128, m: u128) -> u64 {
    if b == 0 {
        return 0;
    }

    let mut r = 1;

    b %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = (r * b) % m;
        }

        e >>= 1;
        b = (b * b) % m;
    }

    r as u64
}