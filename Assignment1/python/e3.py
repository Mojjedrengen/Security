BLOCK_BYTES = 16
MASK = (1 << 128) - 1
MASKI = (1 >> 128) - 1


def rotate_left_13(block: bytes) -> bytes:
    if len(block) != BLOCK_BYTES:
        raise ValueError("Expected exactly 16 bytes")
    value = int.from_bytes(block, "big")
    return (((value << 13) & MASK) | (value >> 115)).to_bytes(16, "big")


def xor(left: bytes, right: bytes) -> bytes:
    if len(left) != BLOCK_BYTES or len(right) != BLOCK_BYTES:
        raise ValueError("Expected exactly 16 bytes per operand")
    return bytes(a ^ b for a, b in zip(left, right))


def encrypt(plaintext: bytes, key1: bytes, key2: bytes) -> bytes:
    return xor(rotate_left_13(xor(plaintext, key1)), key2)


def rotate_left_13i(block: bytes) -> bytes:
    if len(block) != BLOCK_BYTES:
        raise ValueError("Expected exactly 16 bytes")
    value = int.from_bytes(block, "big")
    return (((value << 128-13) & MASK) | (value >> 128-115)).to_bytes(16, "big")



def decrypt(encoded: bytes, key1: bytes, key2: bytes) -> bytes:
    return xor(rotate_left_13i(xor(encoded, key2)), key1) 


if __name__ == "__main__":
    # Public test vector. These are NOT the challenge keys.
    p = bytes(16)
    k1 = bytes.fromhex("00000000000000000000000000000001")
    k2 = bytes(16)
    expected = "00000000000000000000000000002000"
    assert encrypt(p, k1, k2).hex() == expected
    print("Public test vector passed:", expected)
    print("p: ", p.hex())
    encrypted = encrypt(p, k1, k2)
    print("decoded: ", decrypt(encrypted, k1, k2).hex())
    print("k1: ", k1.hex())
    print("k2: ", k2.hex())
