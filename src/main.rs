extern crate secret_sharing_spring2020;
extern crate num;

use num::bigint::BigUint;
use secret_sharing_spring2020::rand_gen::linear_congruent;
use secret_sharing_spring2020::exponentiation::square_multiply;
use secret_sharing_spring2020::prime::factorization;

fn main() {

    let _ar: Vec<BigUint> = linear_congruent::gen(32);
    square_multiply::run();
    // let n = BigUint::from(199842032u32);
    let mut twopow:u32;
    let mut r: BigUint;

    match factorization::get_millrab_d_vec(&_ar) {
        Some(tp) => {
            twopow = tp.power_two;
            r = tp.r;
        }
        None => panic!("Number odd")
    }
    println!("{}, {}", twopow, r);
//    Test

}