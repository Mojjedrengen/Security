use std::collections::{HashMap, HashSet};
use std::println;
use std::time::Instant;

use rand::distr::{Alphanumeric, SampleString};
use sha2::digest::array::Array;
use sha2::digest::consts::{U2, U3, U4};
use sha2::{Digest, Sha256};

fn main() {
    println!("Hello, world!");

    collision();
    collision_const("Hello, world!");
}

fn collision() -> ((String, String), (String, String), (String, String)) {
    let mut h16 = None;
    let mut h24 = None;
    let mut h32 = None;
    let mut h16_seen: HashMap<Array<u8, U2>, String> = HashMap::new();
    let mut h24_seen: HashMap<Array<u8, U3>, String> = HashMap::new();
    let mut h32_seen: HashMap<Array<u8, U4>, String> = HashMap::new();
    let start = Instant::now();
    loop {
        let m1 = rand_string();
        let m2 = rand_string();
        if m1 == m2 {
            continue;
        }
        let h_161 = h_16(&m1);
        let h_162 = h_16(&m2);
        if h16.is_none() {
            if h_161 == h_162 {
                println!("16: ");
                println!("m1: {}", m1);
                println!("m2: {}", m2);
                println!("time: {:#?}", start.elapsed());
                println!(" ");
                h16 = Some((m1.clone(), m2.clone()));
            } else {
                if h16_seen.contains_key(&h_161) {
                    println!("16: ");
                    println!("m1: {}", m1);
                    println!("m2: {}", h16_seen.get(&h_161).unwrap());
                    println!("time: {:#?}", start.elapsed());
                    println!(" ");
                    h16 = Some((m1.clone(), h16_seen.get(&h_161).unwrap().clone()));
                }
                if h16_seen.contains_key(&h_162) {
                    println!("16: ");
                    println!("m1: {}", m1);
                    println!("m2: {}", h16_seen.get(&h_162).unwrap());
                    println!("time: {:#?}", start.elapsed());
                    println!(" ");
                    h16 = Some((m1.clone(), h16_seen.get(&h_162).unwrap().clone()));
                }

                h16_seen.insert(h_161, m1.clone());
                h16_seen.insert(h_162, m2.clone());
            }
        }
        if h24.is_none() {
            let h_241 = h_24(&m1);
            let h_242 = h_24(&m2);
            if h_241 == h_242 {
                println!("24: ");
                println!("m1: {}", m1);
                println!("m2: {}", m2);
                println!("time: {:#?}", start.elapsed());
                println!(" ");
                h24 = Some((m1.clone(), m2.clone()));
            } else {
                if h24_seen.contains_key(&h_241) {
                    println!("24: ");
                    println!("m1: {}", m1);
                    println!("m2: {}", h24_seen.get(&h_241).unwrap());
                    println!("time: {:#?}", start.elapsed());
                    println!(" ");
                    h24 = Some((m1.clone(), h24_seen.get(&h_241).unwrap().clone()));
                }
                if h24_seen.contains_key(&h_242) {
                    println!("24: ");
                    println!("m1: {}", m1);
                    println!("m2: {}", h24_seen.get(&h_242).unwrap());
                    println!("time: {:#?}", start.elapsed());
                    println!(" ");
                    h24 = Some((m1.clone(), h24_seen.get(&h_242).unwrap().clone()));
                }

                h24_seen.insert(h_241, m1.clone());
                h24_seen.insert(h_242, m2.clone());
            }
        }
        if h32.is_none() {
            let h_321 = h_32(&m1);
            let h_322 = h_32(&m2);
            if h_321 == h_322 {
                println!("32: ");
                println!("m1: {}", m1);
                println!("m2: {}", m2);
                println!("time: {:#?}", start.elapsed());
                println!(" ");
                h32 = Some((m1.clone(), m2.clone()));
            } else {
                if h32_seen.contains_key(&h_321) {
                    println!("32: ");
                    println!("m1: {}", m1);
                    println!("m2: {}", h32_seen.get(&h_321).unwrap());
                    println!("time: {:#?}", start.elapsed());
                    println!(" ");
                    h32 = Some((m1.clone(), h32_seen.get(&h_321).unwrap().clone()));
                }
                if h32_seen.contains_key(&h_322) {
                    println!("32: ");
                    println!("m1: {}", m1);
                    println!("m2: {}", h32_seen.get(&h_322).unwrap());
                    println!("time: {:#?}", start.elapsed());
                    println!(" ");
                    h32 = Some((m1.clone(), h32_seen.get(&h_322).unwrap().clone()));
                }

                h32_seen.insert(h_321, m1.clone());
                h32_seen.insert(h_322, m2.clone());
            }
        }
        if let Some(ref h16) = h16
            && let Some(ref h24) = h24
            && let Some(ref h32) = h32
        {
            return (h16.to_owned(), h24.to_owned(), h32.to_owned());
        }
    }
}

fn collision_const(m1: &str) -> (String, String, String) {
    let mut h16 = None;
    let mut h24 = None;
    let mut h32 = None;
    let h_161 = h_16(m1);
    let h_241 = h_24(m1);
    let h_321 = h_32(m1);
    let start = Instant::now();
    loop {
        let m2 = rand_string();
        if m1 == m2 {
            continue;
        }
        if h16.is_none() && h_161 == h_16(&m2) {
            println!("16: ");
            println!("m1: {}", m1);
            println!("m2: {}", m2);
            println!("time: {:#?}", start.elapsed());
            println!(" ");
            h16 = Some(m2.clone());
        }
        if h24.is_none() && h_241 == h_24(&m2) {
            println!("24: ");
            println!("m1: {}", m1);
            println!("m2: {}", m2);
            println!("time: {:#?}", start.elapsed());
            println!(" ");
            h24 = Some(m2.clone());
        }
        if h32.is_none() && h_321 == h_32(&m2) {
            println!("32: ");
            println!("m1: {}", m1);
            println!("m2: {}", m2);
            println!("time: {:#?}", start.elapsed());
            println!(" ");
            h32 = Some(m2.clone());
        }
        if let Some(ref h16) = h16
            && let Some(ref h24) = h24
            && let Some(ref h32) = h32
        {
            return (h16.to_owned(), h24.to_owned(), h32.to_owned());
        }
    }
}

fn rand_string() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 256)
}

fn h_16(m: &str) -> Array<u8, U2> {
    let hash = Sha256::digest(m);
    let (fst, _) = hash.split::<U2>();
    fst
}
fn h_24(m: &str) -> Array<u8, U3> {
    let hash = Sha256::digest(m);
    let (fst, _) = hash.split::<U3>();
    fst
}
fn h_32(m: &str) -> Array<u8, U4> {
    let hash = Sha256::digest(m);
    let (fst, _) = hash.split::<U4>();
    fst
}
