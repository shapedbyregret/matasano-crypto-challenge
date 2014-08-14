import base64
import binascii

the_string = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"


def hex_to_b64(hex_val):
    if isinstance(hex_val, str):
        hex_val = binascii.unhexlify(hex_val)
        b64_val = base64.b64encode(hex_val)
        return b64_val
    return 'err'


if __name__ == "__main__":
    val = hex_to_b64(the_string)
    print(val)
