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

    // Only generate 300; it doesn't take long to repeat
    let num = 300u32;

    let mut n: BigUint;
    let mut p = s.clone();
    let mut ar= Vec::new();

    for _i in 0..num + 1 {
        n = (&a * &p + &c) % &m;
        p = n.clone();
        ar.push(n.clone());
        println!("{},{}", _i, &n);
    }

    ar
}
