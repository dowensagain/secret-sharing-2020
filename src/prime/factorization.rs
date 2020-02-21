use num::BigUint;

pub fn get_d_millrab(n:&BigUint) -> Result<(u32, BigUint), ()> {
    let mut m:BigUint = n.clone();
    let mut two_power:u32 = 0;
    let mut r = BigUint::from(0u32);

    // Get least significant bit
    let mut lsb = last_bit(&m.to_bytes_be()[0]);
    let mut still_even = true;

    if lsb == 1u8 {
        panic!("Number odd!")
    }

    while still_even {
        m = m >> 1;
        two_power += 1;
        lsb = last_bit(&m.to_bytes_be()[0]);
        if lsb == 1 {
            r = m.clone();
            still_even = false;
        }

    }

    Ok((two_power, r))
}

fn last_bit(n:&u8) -> u8 {
    let mut lsb:u8 = (*n | 0b11111110);
    return if lsb == 255u8 {
        1u8
    } else {
        0u8
    }
}