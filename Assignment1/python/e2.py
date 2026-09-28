import hashlib

b = {16, 24, 32}

def H_b(m , b: int): 
    hash = hashlib.sha256(m)
    hash[:b]
