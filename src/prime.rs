use num::BigUint;

use crate::generator;

#[derive(Debug)]
struct MillerRabinD {
    pub r: u32,
    pub d: BigUint
}

pub fn test_miller_rabin(vec:&Vec<BigUint>) -> Option<bool> {
    println!();
    println!("{}", "Finding a prime using Miller-Rabin primality test:");
//    let mut n = n.clone();
    let t = 10;
//    if check_odd(&n) { return None; }
    let d:MillerRabinD;

    match get_millrab_d_vec(&vec) {
        Some(sd) => d = sd,
        None => return None
    };

//    for _ in 1..t {
//        let mut a= generator::linear_congruent(n.bits());
//        let exp = BigUint::from(1u32);
//        a = a.modpow( &exp, &n);
//    }

    Some(true)
}

fn get_millrab_d(n:&BigUint) -> Option<MillerRabinD> {
    let mut d:BigUint = n.clone() - BigUint::from(1u32);
    let mut r:u32 = 0;
    let mut is_even = check_even(&d);

    // Check to make sure the number passed is even
    // If so, LSB = 0
    if !is_even { return None; }

    while is_even {
        d = d >> 1; // We know the LSB is 0. Bitshift right to check the next.
        r += 1; // Increment power of 2
        is_even = check_even(&d); // Check new LSB
    }

    let d = MillerRabinD {
        r,
        d
    };

    Some(d)

}

fn get_millrab_d_vec(vec:&Vec<BigUint>) -> Option<MillerRabinD> {
    for i in vec.iter() {
        println!("{} {}:", "Testing", i);
        match get_millrab_d(i) {
            Some(d) => {
                println!("{}: {}", "r identified", d.r);
                println!("{}: {}", "d identified", d.d);
                return Some(d)
            },
            None => println!("(n-1) odd, skipping...")
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

fn last_bit_big_uint(n:&BigUint) -> u8 {
    let n_bytes = n.to_bytes_le()[0];
    return last_bit_u8(&n_bytes);
}

pub fn check_even(n:&BigUint) -> bool {
    return last_bit_big_uint(n) == 0;
}

pub fn check_odd(n:&BigUint) -> bool {
    return last_bit_big_uint(n) == 1;
}