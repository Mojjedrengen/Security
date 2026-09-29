# Exercise 1

- Code: python/e1.py

## 1a. Recover the private key and plaintext

```python
def getkey(p: int, g: int, PK: int) -> int:
    for x in range(0, p):
        pk = pow(g, x) % p
        if pk == PK :
            return x
    return 0
```

```rust
fn getkey(p: usize, g: usize, PK: usize) -> usize {
    let mut result = 1;
    let mut i = 0;
    while result != PK {
        i += 1;
        result = (result * g) % p;
    }
    i
}
```

My method for getting the private key is just a brute force attack.
For every integer smaller than p i just try to create a public key that matches the known one.
Above is an implementation in both Rust and in Python.
Since i keep trying to create the public key i use that as a way to check my result
Since we now have the private key we can just use standard ElGamal to decrypt the message.
When running the code we get

- private key: 24774
- plaintext: 26000

## 1b. Modify the ciphertext **without the private key**

```python
def modify(c2: int, m: int, mprime: int, p: int) -> int:
    m_m1 = pow(m, -1, p);
    a = mprime * m_m1
    return a * c2 % p
```

Above is a python implementation of the code to modify the text.
Here are `c2` the intercepted cyphertext, `m` the known plaintext, `mprime` the target message and `p` from the public parameters.
To modify the cyphertext we need an $`a = m' * m^(-1)`$.
This `a` we then need to multiply with $c2 mod p$ to get the modified cyphertext at decrypts to mprime.
$`E(m') = a * c2 % p`$

## Evidence to include

Give the recovered key and plaintext, the modified ciphertext, the verification output, and the
reasoning behind both attacks. The report should make clear that the ciphertext-modification
algorithm does not depend on private-key recovery.

# Exercise 2

- Code: e2/src/main.rs (rust project)

## 2a. Implement collision search

In the code the function `collision()` searches for collision for each output length and prints them out.
It utilizes a hash map to search for previous attempts.
And searches using brute force.
It searches using a random string of 256 length.
Since rust implements string as UTF-8 arrays I know that these strings are valid UTF-8 encoded strings.
Bellow is listet the strings that causes a collision and the time it took to find them.
Do note that the time to find the strings may vary and this is not an average time but just the time for this example

- 16:
  - m1: fAAYzxUWK2R6RBNV5MLY0CyfZoHkOGumkEL34ARB5QH1G8StWLhUNSEOzHhqM1qKqQgaD9mL9ito
    oO32QVRQ5NlUSUVe9BKVjsEaT62gHeKPw3mkurzzFQDKbhJo6mmAAwXOKe4mSgoA7Q2gGcltFrSAOsOA
    SrMK0BwwnlAmkJXCRVbRT7Dqz6UK4UQMgi0QVPjSfU2Inc423bFtgS7BHULv1HFp9yYHJRqUlsaJKtAF
    BPOkHa9RSym8A1Wt9xLO
  - m2: B3DLaNMUKWuEh3Lqk85BmSAMvSCIj7Er1cwssEllFs5lz9UJlLIwOjDgn3YI2msy7O23sWD89Eo6
    68xIlLEugGvIv6kcisyTbvW15LsCH5Y3eLFwvQ8ZYa2mCOjv2iU1er8XSlozML1MitSFDnMotNYga3rj
    7FLSiBgdRWfLjPRPts5uRpVuF76J4yLhhfz0U8GfZpDagw5ibWcvz70q5pfzMB0KPal1VsEgcvzTnLZi
    e2CNs39u794wxCGCcMT5
  - time: 41.81147ms

- 24:
  - m1: M6O1sKTL3NBIVRA5whLlKr3pKvmj6Jl4L1zOnKVFsPjgEO6iiONgOwpWOtT0S37CVJdgEhWPg0Us
    GppdstxiWetwCZtkZ5cE9Yo4CQjXgObWajEiciaqZhSAqxvFvEU55IBljbIbUtigeIUkBGGJsKf5tXot
    rO8fTvJ4lgMMTi9ysH4ZdsBjRrfXXkr9pFfsZASFfZVEHJZJjjdACXGVt1qZC81VdzGtbhbzbLLqnqwD
    HrY0QD75GGOPbR9QbGkJ
  - m2: 0Sr2MyC4ve4PbptZSqy0FETkTAG6ehIhfbYLluXa9UWhEcICW8vUXFcWhj1JHbxun9dD74MtFF4F
    2Tpi8sKDT68w3GagKdtiqe7ysoTCpvObPdjLvQNlChhJHzfGa08c5St9ZkyDGpAAxxB81cCr0haB9sOX
    cgA804voY2RyZ5CoGReQpAUKnSa0EsR3V9bxP5djCwnaS2A3RJzAaJqpCwrXlZ6ezDHW7w4Xg0W13I2H
    3KPeZUEFUTJFPbljyC2f
  - time: 342.941019ms

- 32:
  - m1: jfN4Hj0TyOBi10MTtkgqIm4bz0dz8RvUeq0AxRqzi73adjWp5sC8y91SpJcNWyk1pAFYFmVzv7LG
    uoew0qbfH3Mso5yla8cmSvdeLYegU0wl2aeuHAgcq6fmdBQoKaFc5bYOS0eh7iW2ycnpDz5ZwKL4Ffsf
    a192QrOhrbpoBHXK81qNl3ik9PW4JXyxJ0cwftSje5NeGsv1wFMwa9TBs4uaw1rD9sdOTZ70JB4rxWck
    gSldBmT31RK9uyamzspB
  * m2: lOzZ5bQs52J03R6oSkflEfPYIRgPLHYySxskpDKHuBOubduF6RVckVlbVqTsSFHatwMWQay1lPb3
    jUsJSZfxW7ouHwUhdZ6y7Xs5qepqyDmzLGTpsULx5ssMZT9T6NTl9SmhYCZGlI1wGszLLAiJxY1NjWOd
    lYThSzN2jpGipdw5T7evJ2U6xM5oG06EnZGGjM0eV336Pa5jYREtbl3Dgc0tghqdVzPST4ltn55GSDje
    ZDnMkaEFxs6KCH94GkIK
  - time: 9.624639663s

## 2b. Distinguish collision and second-preimage resistance

In the same file I have also implemented collision search with a constant string. `collision_const(m1: &str)`
In my testing they take noticeably longer to find.
It also takes exponentially more time to find when we use more bits of the result.
With it already taking a long time to find both a collision for a fixed string and when we use more bits, I conclude that sha-256 is not broken.
And it should only be considered broken if you do not use all the 256 bits you get.
Since you want to use all the 256 bits normally I would not consider it practically broken.

# Exercise 3

## 3a. Derive decryption

$`Ek_1k_2 (P) = R_L(P ^ k_1) ^ k_2`$

$`Dk_1k_2 (C) = R_R(C ^ k_2) ^ k_1`$
