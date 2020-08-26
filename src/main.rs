use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};
use std::iter::Map;
use std::ops::Deref;
use std::time::Instant;
use std::slice::SliceIndex;
use std::env::temp_dir;

struct Text<'a> {
    hashes: Vec<i128>,
    queries: Vec<(usize, usize)>,
    cache: HashMap<(&'a usize, &'a usize), u8>,
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
    println!("processed {} lines", counter);
}

fn cut_number(number: &i128, i1: u8, i2: u8) -> i128 {
    let number = number.clone() >> i1 as i128;
    let mask = 2i128.pow((i2 - i1) as u32) - 1;
    return number & mask;
}

fn process_queries(text: &mut Text, n_bands: u8, band_bits: u8) {
    let mut candidates: HashMap<usize, HashSet<usize>> = HashMap::new();

    for i in 0..text.hashes.len() {
        candidates.insert(i, HashSet::new());
    }

    for band in 0..n_bands {
        let i1 = band * band_bits;
        let i2 = i1 + band_bits;

        let mut buckets: HashMap<i128, HashSet<usize>> = HashMap::new();

        for (index, hash) in text.hashes.iter().enumerate() {
            let band_int = cut_number(hash, i1.into(), i2.into());
            let mut bucket_content = &mut HashSet::new();

            if buckets.contains_key(&band_int) {
                bucket_content = buckets.get_mut(&band_int).unwrap();

                for id in bucket_content.iter() {
                    candidates.get_mut(&index).unwrap().insert(id.clone());
                    candidates.get_mut(&id).unwrap().insert(index);
                }
            }
            bucket_content.insert(index);
            let elem = bucket_content.clone();
            buckets.insert(band_int, elem);
        }
    }

    let mut processed: usize = 0;
    for (i, k) in text.queries.iter() {
        let hash = text.hashes.get(i.clone()).unwrap();
        let possible_candidates = candidates.get(i).unwrap();

        let mut counter = 0;
        for possible in possible_candidates.into_iter() {
            let possible_hash = text.hashes.get(possible.clone()).unwrap();
            let ordered = order(i, possible);

            let mut distance: u8;
            if text.cache.contains_key(&ordered) {
                distance = text.cache.get(&ordered).unwrap().clone();
            } else {
                distance = find_distance(hash, possible_hash);
            }

            if distance < k.clone() as u8 {
                counter += 1;
            }
        }
        processed += 1;
        println!("{}", counter);
    }
    println!("processed {} queries", processed)
}

fn find_distance(hash1: &i128, hash2: &i128) -> u8 {
    let hash1 = hash1.clone();
    let hash2 = hash2.clone();

    let mut result = 0;
    for _ in 0..128 {
        let x1 = hash1 & 1;
        let x2 = hash2 & 1;
        if x1 != x2 {
            result += 1;
        }
    }
    return result;
}

fn order<'a>(i1: &'a usize, i2: &'a usize) -> (&'a usize, &'a usize) {
    if i1 >= i2 {
        return (i1, i2);
    }
    return (i2, i1);
}

fn main() {
    println!("simhash: {:x}", simhash("fakultet elektrotehnike i racunarstva"));
    let mut txt = Text { queries: Vec::new(), hashes: Vec::new(), cache: HashMap::new() };
    let start = Instant::now();
    process_input(&mut txt);
    println!("process_input done in {:?}", start.elapsed());
    process_queries(&mut txt, 8, 16);
    println!("all done in {:?}", start.elapsed());
}
