import base64
import binascii
import math
import string

str1 = '1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736'
all_chars = string.printable
alpha = string.ascii_lowercase
predefined_freqs = {
    'e': 12.02,
    't': 9.1,
    'a': 8.12,
    'o': 7.68,
    'i': 7.31,
    'n': 6.95,
    's': 6.28,
    'r': 6.02,
    'h': 5.92,
    'd': 4.32,
    'l': 3.98,
    'u': 2.88,
    'c': 2.71,
    'm': 2.61,
    'f': 2.3
}


def hex_to_b64(hex_val):
    if isinstance(hex_val, str):
        hex_val = binascii.unhexlify(hex_val)
        b64_val = base64.b64encode(hex_val)
        return b64_val
    return 'err'


# Takes in two hex strings and xors them
# Throws error on this challenge, so unusable.
def xor(sx, sy):
    return '{:x}'.format(int(sx, 16) ^ int(sy, 16))


# Modified from 1-2.py to xor each byte by single byte.
# Also return string instead of hex-string.
def xor2(sx, sy):
    sx = binascii.unhexlify(sx)
    s = "".join(chr(x ^ ord(sy)) for x in sx)
    # return "".join('{:x}'.format(ord(c)) for c in s)
    return s


def check_freq(freqs):
    cmps = []
    for c in predefined_freqs:
        if c in freqs:
            cmps.append(math.fabs(freqs[c] - predefined_freqs[c]))
            # Find larger number
            '''if freqs[c] > predefined_freqs[c]:
                cmps.append(freqs[c] - predefined_freqs[c])
            else:
                cmps.append(predefined_freqs[c] - freqs[c])'''
        else:
            # Penalize by arbitrary value if character not found in predefined
            # frequency list
            cmps.append(100)
    score = sum(cmps)
    return score


def char_freq(s):
    occurrences = {}
    for c in s:
        if c in occurrences:
            occurrences[c] += 1
        else:
            occurrences[c] = 1
    freqs = {}
    str_len = len(s)
    for k, v in occurrences.items():
        freqs[k] = (v / str_len) * 100
    return freqs


if __name__ == "__main__":
    # XOR given string with a single character
    # then determine probability of being english
    minscore = 999999
    unxored_word = ''
    unxored_key = ''
    for x in all_chars:
        unxored = xor2(str1, x)
        # Originally lowered and removed non-alpha characters but turned out
        # to be unnecessary
        '''
        lowered = unxored.lower()
        clean_lowered = ''
        # strip non-alpha characters
        for c in lowered:
            if c in alpha:
                clean_lowered += c
        # skip if string is empty
        if not clean_lowered:
            continue
        freqs = char_freq(clean_lowered)
        '''
        freqs = char_freq(unxored)
        score = check_freq(freqs)
        if score < minscore:
            minscore = score
            unxored_word = unxored
            unxored_key = x
    print(minscore)
    print(unxored_word)
    print(unxored_key)
