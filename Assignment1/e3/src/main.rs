fn main() {
    println!("Hello, world!");
    let p = 0x00000000000000000000000000000000;
    let k1 = 0x00000000000000000000000000000001;
    let k2 = 0x00000000000000000000000000000000;
    let expected = 0x00000000000000000000000000002000;
    let encryped = encrypt(p, k1, k2);
    println!("Encrypted: {:#x}", encryped);
    println!("Expected: {:#x}", expected);
    println!("Equals: {}", expected == encryped);
    let plain = decrypt(encryped, k1, k2);
    println!("Decrypted: {:#x}", plain);
    println!("Expected: {:#x}", p);
    println!("Equals: {}", plain == p);
}

fn encrypt(plaintext: u128, key1: u128, key2: u128) -> u128 {
    let r = (plaintext ^ key1).rotate_left(13);
    r ^ key2
}

fn decrypt(cypertext: u128, key1: u128, key2: u128) -> u128 {
    let r = cypertext ^ key2;
    let pk = r.rotate_right(13);
    pk ^ key1
}

fn get_key_mask(p0: u128, c0: u128) -> u128 {
    p0.rotate_left(13) ^ c0
}
fn decrypt_with_mask(cyphertext: u128, mask: u128) -> u128 {
    (cyphertext ^ mask).rotate_right(13)
}

/*
* {
*  "block_bytes": 16,
*  "encoding": "128-bit big-endian; exact bytes; no padding",
*  "known_plaintext_hex": "6b6e6f776e2d626c6f636b2d30303031",
*  "known_ciphertext_hex": "0e2061c487fe8e09eda7238088d1a6ea",
*  "challenges": [
*    {
*      "id": "C1",
*      "ciphertext_hex": "4d6200ace534a5a346c483828ad385e9"
*    },
*    {
*      "id": "C2",
*      "ciphertext_hex": "8fe200a68d9547e184c681828ad38769"
*    },
*    {
*      "id": "C3",
*      "ciphertext_hex": "6d4381ef669d6f49076f48628ad387a9"
*    }
*  ],
*  "target_plaintext_hex": "73747564656e743d3132333435202020"
* }
*/
