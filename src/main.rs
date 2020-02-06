extern crate secret_sharing_spring2020;

use secret_sharing_spring2020::rand_gen::linear_congruent;
use secret_sharing_spring2020::exponentiation::square_multiply;

fn main() {

    let _ar: Vec<i128> = linear_congruent::gen(1000);
    square_multiply::run();

//    Test

}