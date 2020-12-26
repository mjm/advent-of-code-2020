const CARD_PUBLIC_KEY: u64 = 15113849;
const DOOR_PUBLIC_KEY: u64 = 4206373;

fn main() {
    let card_loop_size = calculate_loop_size(CARD_PUBLIC_KEY, 7);
    let door_loop_size = calculate_loop_size(DOOR_PUBLIC_KEY, 7);

    let encryption_key1 = transform_value(CARD_PUBLIC_KEY, door_loop_size);
    let encryption_key2 = transform_value(DOOR_PUBLIC_KEY, card_loop_size);
    assert_eq!(encryption_key1, encryption_key2);
    println!("The encryption key is {}", encryption_key1);
}

fn transform_value(subject_num: u64, loop_size: u64) -> u64 {
    let mut value = subject_num;

    for _ in 0..loop_size {
        value = (value * subject_num) % 20201227;
    }

    value
}

fn calculate_loop_size(key: u64, subject_num: u64) -> u64 {
    let mut loop_size = 1;
    let mut value = key;

    loop {
        value = (value * mod_inv(subject_num, 20201227)) % 20201227;
        if value == subject_num {
            return loop_size;
        }

        loop_size += 1;
    }
}

// compute the modular multiplicative inverse of a mod n using extended Euclidean algorithm
fn mod_inv(a: u64, n: u64) -> u64 {
    let (mut old_r, mut r) = (a, n);
    let (mut old_s, mut s): (i64, i64) = (1, 0);

    while r != 0 {
        let quotient = old_r / r;

        let new_r = old_r - quotient * r;
        old_r = r;
        r = new_r;

        let new_s = old_s - (quotient as i64) * s;
        old_s = s;
        s = new_s;
    }

    if old_s < 0 {
        n - (old_s.abs() as u64)
    } else {
        old_s as u64
    }
}