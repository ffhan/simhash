use std::io::stdin;

use md5::Digest;

struct Text {
    hashes: Vec<[u8; 128]>,
    queries: Vec<(u64, u64)>,
}

fn update_sh(hash: &[u8; 16], sh: &mut [i64; 128]) {
    for j in 0..hash.len() {
        let i = hash.len() - 1 - j;
        let mut x = hash[i];
        for b in 0..8 {
            let bit = 7 - b;
            if x & 1 == 1 {
                sh[8 * i + bit] += 1;
            } else {
                sh[8 * i + bit] -= 1;
            }
            x = x >> 1;
        }
    }
}

fn bin_to_int(sh: &[u8; 128]) -> i128 {
    println!("{:?}", sh.to_vec());
    let mut result: i128 = 0;
    for i in 0..sh.len() {
        result = (result << 1) | (sh[i] as i128)
    }
    return result;
}

fn simhash(line: &str) -> [u8; 128] {
    let mut sh: [i64; 128] = [0; 128];
    let mut result: [u8; 128] = [0; 128];
    let split: Vec<&str> = line.split(' ').collect();
    for i in 0..split.len() {
        let word = split[i];
        let hash = md5::compute(word);
        update_sh(&hash, &mut sh);
    }
    for i in 0..sh.len() {
        let index = sh.len() - 1 - i;
        if sh[index] >= 0 {
            result[index] = 1;
        } else {
            result[index] = 0;
        }
    };
    return result;
}

fn process_input(text: &mut Text) {
    let mut line = String::new();
    match stdin().read_line(&mut line) {
        Ok(_) => {
            text.hashes.append(&mut vec![simhash(&line)])
        }
        Err(err) => {
            panic!(err)
        }
    }
}

fn main() {
    println!("simhash: {:x}", bin_to_int(&simhash("fakultet elektrotehnike i racunarstva")))
}
