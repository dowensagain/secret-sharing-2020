use num::bigint::BigUint;
use num::pow;
use std::time::SystemTime;

pub fn linear_congruent(bit_length:usize) -> BigUint {
    // Generate a "random" seed from system time
    let time:u128;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => time = n.as_nanos(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    // Define constants
    let m = pow(BigUint::from(2u32), bit_length);
    let a = pow(BigUint::from(3u32), bit_length/2);
    let mut p = pow(BigUint::from(time), bit_length/2);
    let c = BigUint::from(1u32);
    let mut unfinished = true;
    let mut n =  BigUint::from(0u32);

    while unfinished {
        n = (&a * &p + &c) % &m;
        // Only store the number if it matches our bit length, and is not a repeat
        if (n.bits() == bit_length) && (p != n) {
            unfinished = false;
        }
        p = n.clone();
    }
    n

}

// Due to how the random generation requires previously generated
// values, it is necessary to create two functions

pub fn many_linear_congruent(bit_length:usize, num_to_gen: u32) -> Vec<BigUint> {
    println!("{} {} {}{}", "Generating", num_to_gen, bit_length, "-bit numbers:");
    // Generate a "random" seed from system time
    let time:u64;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => time = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    let m = pow(BigUint::from(2u32), bit_length);
    let a = pow(BigUint::from(3u32), bit_length/2);
    let mut p = pow(BigUint::from(time), bit_length/2);
    let c = BigUint::from(1u32);
    let mut ar= Vec::new();
    let mut unfinished = true;
    let mut i  = 0;
    let mut n:BigUint;


    while unfinished {
        n = (&a * &p + &c) % &m;
        // Only store the number if it matches our bit length, and is not a repeat
        if (n.bits() == bit_length) && (p != n) {
            ar.push(n.clone());
            println!("{}: {}", i, &n);
            i += 1;
            if i > num_to_gen { unfinished = false; }
        }
        p = n.clone();
    }

    ar

}
