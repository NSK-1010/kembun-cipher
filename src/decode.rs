use std::fs::File;
use std::io::{BufReader, BufRead};
use kryptos::ciphers::railfence::RailFence;
use std::io::Cursor;
use zstd;

pub fn decode(path: &str) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut rail1: usize = 0;
    let mut rail2: usize = 0;
    let mut input: Vec<isize> = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        match i {
            0 => rail1 = line.parse().unwrap(),
            1 => rail2 = line.parse().unwrap(),
            2 => input = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
            _ => (),
        }
    }

    let mut railfence2 = String::new();
    for i in 0..input.len() {
        if input[i] == 0 {
            railfence2.push('.');
        } else if input[i] > 0 {
            for _ in 0..input[i] {
                railfence2.push('+');
            }
        } else if input[i] < 0 {
            for _ in 0..-input[i] {
                railfence2.push('-');
            }
        }
    }
    let bf = RailFence::new(rail2).unwrap().decipher(&railfence2).unwrap();
    let mut railfence1_u8: Vec<u8> = vec![];
    let mut bf_sum: u8 = 0;
    let bf_char: Vec<char> = bf.chars().collect();
    for i in 0..bf_char.len() {
        match bf_char[i] {
            '+' => bf_sum += 1,
            '-' => bf_sum -= 1,
            '.' => {
                railfence1_u8.push(bf_sum);
            },
            _ => (),
        };
    }
    let railfence1: &str = &String::from_utf8(railfence1_u8).unwrap();
    let mergetext = RailFence::new(rail1).unwrap().decipher(railfence1).unwrap();
    let merged = mergetext.as_bytes();
    let (div1, div2) = merged.split_at(merged.len() / 2);
    let diffed: Vec<u8> = div1.to_vec();
    let zstded_index: Vec<u8> = div2.to_vec();
    let mut sorted = vec![];
    for i in 0..diffed.len() {
        if i == 0 {
            sorted.push(diffed[i]);
        } else {
            sorted.push(sorted[i-1] + diffed[i]);
        }
    }
    let mut zstded: Vec<u8> = vec![0; sorted.len()];
    for (i, &x) in zstded_index.iter().enumerate() {
        zstded[x as usize] = sorted[i];
    }
    let cleartext_byte = zstd::stream::decode_all(Cursor::new(zstded)).unwrap();
    let cleartext = std::str::from_utf8(&cleartext_byte).unwrap();
    println!("{}", cleartext);
}
