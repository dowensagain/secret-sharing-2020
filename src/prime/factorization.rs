use num::BigUint;


pub fn get_millrab_d(n:&BigUint) -> Result<(u32, BigUint), ()> {
    let mut m:BigUint = n.clone();
    let mut two_power:u32 = 0;
    let mut r = BigUint::from(0u32);
    let mut still_even = is_even(&m);

    if !still_even {
        panic!("Number odd!")
    }

    while still_even {
        m = m >> 1;
        two_power += 1;
        still_even = is_even(&m);
    }

    r = m.clone();

    Ok((two_power, r))
}

fn last_bit_u8(n:&u8) -> u8 {
    let lsb:u8 = *n | 0b11111110;
    return if lsb == 255u8 {
        1u8
    } else {
        0u8
    }
}

fn last_bit_BigUint(n:&BigUint) -> u8 {
    let n_bytes = n.to_bytes_le()[0];
    let lsb = last_bit_u8(&n_bytes);
    lsb
}

fn is_even(n:&BigUint) -> bool {
    return last_bit_BigUint(n) == 0;
}