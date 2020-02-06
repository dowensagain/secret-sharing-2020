
pub fn gen(num:i32) -> Vec<i128> {
    let s: i128 = 1001;
    let m: i128 = i128::pow(2, 64);
    let a: i128 = 22695477;
    let c: i128 = 1;
    let mut n:i128 = s;
    let mut p:i128 = n;
    let mut ar:Vec<i128> = vec![0];

    for _i in 0..num - 1 {
        n = (a * p + c) % m;
        p = n;
        ar.push(n);
        println!("{}", n);
    }

    ar
}
