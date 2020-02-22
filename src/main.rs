extern crate secret_sharing_spring2020;
extern crate num;

use num::bigint::BigUint;
use secret_sharing_spring2020::generator;
use secret_sharing_spring2020::exponentiation;
use secret_sharing_spring2020::prime;

fn main() {

    let _ar: Vec<BigUint> = generator::many_linear_congruent(4096, 10);

    prime::test_miller_rabin(&_ar[0]);
    // let n = BigUint::from(199842032u32);

//    let mut twopow:u32;
//    let mut r: BigUint;
//
//    match factorization::get_millrab_d_vec(&_ar) {
//        Some(tp) => {
//            twopow = tp.power_two;
//            r = tp.r;
//        }
//        None => panic!("Number odd")
//    }
//    println!("{}, {}", twopow, r);
//    Test

}