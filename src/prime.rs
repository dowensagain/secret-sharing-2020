use num::BigUint;
use num::pow;

use crate::generator;

pub struct MillerRabinD {
    pub power_two: u32,
    pub r: BigUint
}

pub fn test_miller_rabin(n:&BigUint) -> Option<bool> {
    let mut n = n.clone();
    let mut t = 10;
    if check_odd(&n) { return None; }

    let n1 = n.clone() - BigUint::from(1u32);
    let mut d:MillerRabinD;

    match get_millrab_d(&n1) {
        Some(sd) => d = sd,
        None => return None
    };

    for i in 1..t {
        let mut a= generator::linear_congruent(n.bits());
        let exp = BigUint::from(1u32);
        a = a.modpow( &exp, &n);
    }

    Some(true)
}

fn get_millrab_d(n:&BigUint) -> Option<MillerRabinD> {
    let mut r:BigUint = n.clone();
    let mut power_two:u32 = 0;
    let mut is_even = check_even(&r);

    // Check to make sure the number passed is even
    // If so, LSB = 0
    if !is_even { return None; }

    while is_even {
        r = r >> 1; // We know the LSB is 0. Bitshift right to check the next.
        power_two += 1; // Increment power of 2
        is_even = check_even(&r); // Check new LSB
    }

    let d = MillerRabinD {
        power_two,
        r
    };

    Some(d)

}

fn get_millrab_d_vec(vec:&Vec<BigUint>) -> Option<MillerRabinD> {
    for i in vec.iter() {
        match get_millrab_d(i) {
            Some(d) => return Some(d),
            None => println!("Odd, skipping...")
        }
    }
    None
}

fn last_bit_u8(n:&u8) -> u8 {
    let lsb:u8 = n | 0b11111110;
    return if lsb == 255u8 {
        1u8
    } else {
        0u8
    }
}

fn last_bit_BigUint(n:&BigUint) -> u8 {
    let n_bytes = n.to_bytes_le()[0];
    return last_bit_u8(&n_bytes);
}

pub fn check_even(n:&BigUint) -> bool {
    return last_bit_BigUint(n) == 0;
}

pub fn check_odd(n:&BigUint) -> bool {
    return last_bit_BigUint(n) == 1;
}