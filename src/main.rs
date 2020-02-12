extern crate secret_sharing_spring2020;
extern crate num;

use num::bigint::BigUint;
use secret_sharing_spring2020::rand_gen::linear_congruent;
use secret_sharing_spring2020::exponentiation::square_multiply;

fn main() {

    let _ar: Vec<BigUint> = linear_congruent::gen(4096);
    square_multiply::run();

//    Test

}