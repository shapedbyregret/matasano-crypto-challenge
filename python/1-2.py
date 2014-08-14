import base64
import binascii

str1 = '1c0111001f010100061a024b53535009181c'
str2 = '686974207468652062756c6c277320657965'


def hex_to_b64(hex_val):
    if isinstance(hex_val, str):
        hex_val = binascii.unhexlify(hex_val)
        b64_val = base64.b64encode(hex_val)
        return b64_val
    return 'err'


# Takes in two hex strings and xors them
def xor(sx, sy):
    return '{:x}'.format(int(sx, 16) ^ int(sy, 16))


def xor2(sx, sy):
    sx = binascii.unhexlify(sx)
    sy = binascii.unhexlify(sy)
    s = "".join(chr(x ^ y) for x, y in zip(sx, sy))
    return "".join('{:x}'.format(ord(c)) for c in s)


if __name__ == "__main__":
    val = xor(str1, str2)
    print(val)
    val = xor2(str1, str2)
    print(val)