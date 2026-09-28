import math
import sys
from typing import Optional
sys.set_int_max_str_digits(10000000)

g = 42
p = 29837
PK = 22690
c1 = 23447
c2 = 8372

"""
# a1
"""
"""Extended Euclidean algorithm"""
def gcd(a, b):
    while b:
        a, b = b, a % b 
    return a
def modInverse(n, m):
    if gcd(n, m) != 1:
        return -1
    m0 = m 
    y = 0
    x = 1 
    if m == 1: 
        return 0
    while n > 1:
        q = n // m
        t = m 
        m = n % m 
        n = t 
        t = y 
        y = x - q * y 
        x = t
    if x < 0:
        x = x + m0
    return x

"""Iteratively brute forces the private key"""
def getkey(p: int, g: int, PK: int) -> int:
    for x in range(0, p):
        pk = pow(g, x) % p  
        if pk == PK :
            return x
    return 0
 
"""Uses standard ElGamal decryption"""
def decrypt(x: int, p: int, c1: int, c2: int) -> int: 
    s = pow(c1, x) % p
    #s_m1 = gcd(s, p) % p
    #s_m1 = (pow(c1, p-x)) % p
    s_m1 = modInverse(s, p)
    m = c2 * s_m1 % p
    return m


x = getkey(p, g, PK);
m = decrypt(x, p, c1, c2) % p;
print("1a")
print("Private key/x: ",x)
print("Plaintext/m: ", m)
print()


"""
# 1b
"""


for x in range(0, p):
    c2n = x * c2 % p
    if decrypt(x, p, c1, c2n) == 12345:
        print("nx: ", x)
        print("c2n: ", c2n)
        print("decrypt: ", decrypt(x, p, c1, c2n))
