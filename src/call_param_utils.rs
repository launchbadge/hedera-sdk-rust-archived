// Helper functions for call_params

pub(crate) fn check_fixed_array_len<T>(arr: &[T], fixed_len: usize) {
    if arr.len() != fixed_len {
        panic!("ILLEGAL ARGUMENT ERROR: fixedLen ({:#?}) does not match array length \
        ({:#?})", fixed_len, arr.len());
    }
}

pub(crate) fn check_int_width(width: usize) {
    if width % 8 != 0 || width < 8 || width > 256 {
        panic!("ILLEGAL ARGUMENT ERROR: Solidity integer width must be a multiple of 8, in the \
        closed range [8, 256]");
    }
}

pub(crate) fn create_padding() -> Vec<u8> {
    let pad = vec![0u8; 32];
    pad
}

pub(crate) fn create_negative_padding() -> Vec<u8> {
    let pad = vec![0xFFu8; 32];
    pad
}

pub(crate) fn left_pad(input: Vec<u8>, negative: bool) -> Vec<u8> {
    let rem = 32 - input.len() % 32;
    if rem == 32 { return input }

    let mut padding = if negative == true { create_negative_padding() } else { create_padding() };
    padding = padding[..rem].to_vec();
    padding.extend(input);
    padding
}

pub(crate) fn right_pad(input: Vec<u8>) -> Vec<u8> {
    let rem = 32 - input.len() % 32;
    if rem == 32 { return input }

    let mut padding = input;
    padding.extend(create_padding()[..rem].to_vec());
    padding
}

pub(crate) fn int256(val: isize) -> Vec<u8> {
    let bytes: [u8; 8] = val.to_be_bytes();
    let padded_bytes = left_pad(bytes.to_vec(), val < 0);
    padded_bytes
}

pub(crate) fn uint256(val: usize) -> Vec<u8> {
    let padded_bytes = left_pad(val.to_be_bytes().to_vec(), false);
    padded_bytes
}

pub(crate) fn encode_bytes(b: Vec<u8>) -> Vec<u8> {
    let mut pad = int256(b.len() as isize);
    let bytes = right_pad(b);
    pad.extend(bytes);
    pad
}

pub(crate) fn encode_string(string: String) -> Vec<u8> {
    let b = string.as_bytes().to_vec();
    let pad = encode_bytes(b);
    pad
}

pub(crate) fn encode_fixed_bytes(b: Vec<u8>) -> Vec<u8> {
    let pad = right_pad(b);
    pad
}

pub(crate) fn encode_byte_array(byte_array: Vec<Vec<u8>>, prepend_len: bool) -> Vec<u8> {
    let mut bytes = Vec::new();
    for b in byte_array.iter() {
        bytes.extend(b);
    }

    if prepend_len == true {
        let mut enc_bytes = int256(byte_array.len() as isize);
        enc_bytes.extend(bytes);
        return enc_bytes
    }
    bytes
}

pub(crate) fn encode_int_array(int_array: Vec<isize>, int_width: usize, prepend_len: bool) -> Vec<u8> {
    check_int_width(int_width);

    let mut bytes = Vec::new();
    for i in int_array.iter() {
        let enc_i = int256(*i);
        bytes.extend(enc_i);
    }

    if prepend_len == true {
        let mut enc_bytes = int256(int_array.len() as isize);
        enc_bytes.extend(bytes);
        return enc_bytes
    }
    bytes
}

pub(crate) fn encode_uint_array(int_array: Vec<usize>, int_width: usize, prepend_len: bool) -> Vec<u8> {
    check_int_width(int_width);

    let mut bytes = Vec::new();
    for i in int_array.iter() {
        let enc_i = uint256(*i);
        bytes.extend(enc_i);
    }

    if prepend_len == true {
        let mut enc_bytes = int256(int_array.len() as isize);
        enc_bytes.extend(bytes);
        return enc_bytes
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::string::ToString;
    use std::iter::Iterator;

    #[test]
    fn test_int256_encoding() {
        let enc_vals = vec![
            "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
            "00000000000000000000000000000000000000000000000000000000000000ff".to_string(),
            "0000000000000000000000000000000000000000000000000000000000000fff".to_string(),
            "000000000000000000000000000000000000000000000000000000007f000000".to_string(),
            "000000000000000000000000000000000000000000000000000000007ff00000".to_string(),
            "00000000000000000000000000000000000000000000000000000000deadbeef".to_string(),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string(),
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe".to_string(),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00".to_string(),
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000".to_string(),
        ];
        let vals = vec![0, 2, 255, 4095, 127 << 24, 2047 << 20, 0xdeadbeefi64, -1, -2, -256,
                        -4096];

        let test_set: HashMap<_, _> = enc_vals.iter().zip(vals.iter()).collect();

        for (k, v) in &test_set {
            assert_eq!(hex::encode(int256(*v.clone() as isize)), *k.clone())
        }

        // The initial bit size for the left shift operations below is critical.
        // They need to be left outside the vector so that they are not forced into an incorrect size.
        assert_eq!(
            hex::encode(int256((255 << 24) as isize)),
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffff000000".to_string()
        );

        assert_eq!(
            hex::encode(int256((4095 << 20) as isize)),
            "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00000".to_string()
        );
    }
}
