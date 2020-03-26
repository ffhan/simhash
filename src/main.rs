use std::io::{BufRead, stdin};
use std::time::Instant;

struct Text {
    hashes: Vec<i128>,
    queries: Vec<(usize, usize)>,
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

fn simhash(line: &str) -> i128 {
    let mut sh: [i64; 128] = [0; 128];
    let mut result: i128 = 0;
    let split: Vec<&str> = line.split(' ').collect();
    for i in 0..split.len() {
        let word = split[i];
        let hash = md5::compute(word);
        update_sh(&hash, &mut sh);
    }
    for i in 0..sh.len() {
        if sh[i] >= 0 {
            result = (result << 1) | 1;
        } else {
            result = (result << 1) | 0;
        }
    };
    return result;
}

fn process_input(text: &mut Text) {
    let mut counter: usize = 0;
    let mut n: usize = 0;
    let mut q: usize = 0;
    for res in stdin().lock().lines() {
        match res {
            Ok(line) => {
                if counter == 0 {
                    n = line.parse::<usize>().unwrap();
                } else if counter <= n {
                    text.hashes.append(&mut vec![simhash(&line)]);
                } else if counter == n + 1 {
                    q = line.parse::<usize>().unwrap();
                } else {
                    let split: Vec<&str> = line.split(' ').collect();
                    let i = split[0].parse::<usize>().unwrap();
                    let k = split[1].parse::<usize>().unwrap();
                    text.queries.append(&mut vec![(i, k)])
                }
                counter += 1;
            }
            Err(err) => {
                panic!(err);
            }
        }
    }
}

fn main() {
    // println!("simhash: {:x}", simhash("fakultet elektrotehnike i racunarstva"))
    let mut txt = Text { queries: Vec::new(), hashes: Vec::new() };
    let start = Instant::now();
    process_input(&mut txt);
    println!("process_input done in {:?}", start.elapsed())
}
