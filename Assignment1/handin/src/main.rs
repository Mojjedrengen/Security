use std::println;

use curv::BigInt;
use curv::arithmetic::Modulo;

fn main() {
    println!("Hello, world!");
    let g = BigInt::from(42);
    let pp = 29837;
    let p = BigInt::from(pp);
    let PK = BigInt::from(22690);
    let c1 = BigInt::from(23447);
    let c2 = BigInt::from(8372);
    let x = getkey(&p, pp, &g, &PK);
    println!("x: {x}");
    let m = decrypt(&x, &p, &c1, &c2);
    println!("m: {m}");
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn getkey(p: &BigInt, pp: u64, g: &BigInt, PK: &BigInt) -> BigInt {
    for x in 0..pp {
        let x = BigInt::from(x);
        let pk = BigInt::mod_pow(g, &x, p);
        if pk == *PK {
            return x;
        }
    }
    BigInt::from(-1)
}
fn getkey2(p: usize, g: usize, PK: usize) -> usize {
    let mut result = 1;
    let mut i = 0;
    while result != PK {
        i += 1;
        result = (result * g) % p;
    }
    i
}

fn decrypt(x: &BigInt, p: &BigInt, c1: &BigInt, c2: &BigInt) -> BigInt {
    let s = BigInt::mod_pow(c1, x, p);
    let s_m1 = BigInt::mod_inv(&s, p);
    BigInt::mod_mul(c2, &s_m1.unwrap(), p)
}
