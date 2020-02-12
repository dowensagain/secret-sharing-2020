use num::bigint::BigUint;
use num::pow;
use std::time::SystemTime;

pub fn gen(bit_length:usize) -> Vec<BigUint> {

    // Generate a "random" seed from system time
    let time:u64;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => time = n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }

    // Define constants
    let s= pow(BigUint::from(time), bit_length/2);
    let m = pow(BigUint::from(2u32), bit_length);
    let a = pow(BigUint::from(3u32), bit_length/2);
    let c = BigUint::from(1u32);

    let num_to_gen:u32 = 100; // Only generate 100; it doesn't take long to repeat
    let mut unfinished = true;
    let mut _i  = 0;
    let mut n: BigUint;
    let mut p = s.clone();
    let mut ar= Vec::new();

    while unfinished {
        n = (&a * &p + &c) % &m;
        // Only store the number if it matches our bit length, and is not a repeat
        if (n.bits() == bit_length) && (p != n) {
            ar.push(n.clone());
            println!("{},{}", _i, &n);
            _i += 1;
            if _i > num_to_gen { unfinished = false; }
        }
        p = n.clone();
    }

    ar
}
