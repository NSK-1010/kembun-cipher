use rand::prelude::*;
use kryptos::ciphers::railfence::RailFence;
use zstd;
use std::fs::File;
use std::io::Write;

pub fn encode(cleartext: &str) -> std::io::Result<()> {
    let mut rng = rand::thread_rng();
    let zstded = zstd::stream::encode_all(cleartext.as_bytes(), 1).unwrap();
    let mut sorted: Vec<u8> = zstded.clone();
    sorted.sort();
    let mut zstded_index: Vec<u8> = vec![];
    for i in 0..sorted.len() {
        for j in 0..zstded.len() {
            if zstded_index.contains(&j.try_into().unwrap()) {
                continue;
            }
            if sorted[i] == zstded[j] {
                zstded_index.push(j.try_into().unwrap());
            }
        }
    }
    let mut diffed: Vec<u8> = vec![];
    for i in 0..sorted.len() {
        if i == 0 {
            diffed.push(sorted[i])
        } else {
            diffed.push(sorted[i] - sorted[i-1])
        }
    }
    let mut merged: Vec<u8> = Vec::new();
    merged.extend(diffed);
    merged.extend(zstded_index);
    let mergetext = &String::from_utf8(merged).unwrap();
    let rail1: usize = rng.gen::<usize>() % 18 + 2;
    let railfence1 = RailFence::new(rail1).unwrap().encipher(mergetext).unwrap();
    let railfence1_u8 = railfence1.as_bytes();
    let mut bf_string = String::new();
    for i in 0..railfence1_u8.len() {
        if i == 0 {
            for _ in 0..railfence1_u8[i] {
                bf_string.push('+');
            }
        } else if railfence1_u8[i] < railfence1_u8[i-1] {
            for _ in 0..railfence1_u8[i-1]-railfence1_u8[i] {
                bf_string.push('-');
            }
        } else if railfence1_u8[i] > railfence1_u8[i-1] {
            for _ in 0..railfence1_u8[i]-railfence1_u8[i-1] {
                bf_string.push('+');
            }
        }
        bf_string.push('.');
    }
    let bf: &str = &bf_string;
    let rail2: usize = rng.gen::<usize>() % 18 + 2;
    let railfence2 = RailFence::new(rail2).unwrap().encipher(bf).unwrap();
    let railfence2_char: Vec<char> = railfence2.chars().collect();
    let mut pm = 0;
    let mut result: Vec<isize> = vec![];
    for i in 0..railfence2_char.len() {
        match railfence2_char[i] {
            '.' => {
                result.push(0);
                pm = 0;
            }
            '+' => {
                pm += 1;
                if i == railfence2_char.len() - 1 || railfence2_char[i + 1] != '+' {
                    result.push(pm);
                    pm = 0;
                }
            }
            '-' => {
                pm -= 1;
                if i == railfence2_char.len() - 1 || railfence2_char[i + 1] != '-' {
                    result.push(pm);
                    pm = 0;
                }
            }
            _ => (),
        }
    }
    let mut file = File::create("decode.txt")?;
    let result_str = result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
    writeln!(file, "{}", rail1)?;
    writeln!(file, "{}", rail2)?;
    writeln!(file, "{}", result_str)?;
    println!("{}", cleartext);
    println!("{}", rail1);
    println!("{}", rail2);
    println!("{}: {:?}", result.len(), result);

    Ok(())
}