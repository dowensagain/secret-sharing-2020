extern crate secret_sharing_spring2020;
extern crate num;

use num::bigint::BigUint;
use secret_sharing_spring2020::rand_gen::linear_congruent;
use secret_sharing_spring2020::exponentiation::square_multiply;
use secret_sharing_spring2020::prime::factorization;

fn main() {

    let _ar: Vec<BigUint> = linear_congruent::gen(4);
    square_multiply::run();
    let n = BigUint::from(112u32);
    let mut twopow = 0u32;
    let mut r = BigUint::from(0u32);

    match factorization::get_d_millrab(&n) {
        Ok(tp) => {
            twopow = tp.0;
            r = tp.1;
        }
        Err(_) => panic!("Number odd")
    }
//    Test

}