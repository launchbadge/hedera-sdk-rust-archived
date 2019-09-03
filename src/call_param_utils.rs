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

pub(crate) fn int256(val: i64) -> Vec<u8> {
    let bytes: [u8; 8] = val.to_be_bytes();
    let padded_bytes = left_pad(bytes.to_vec(), val < 0);
    padded_bytes
}

pub(crate) fn uint256(val: u64) -> Vec<u8> {
    let padded_bytes = left_pad(val.to_be_bytes().to_vec(), false);
    padded_bytes
}

pub(crate) fn encode_bytes(b: Vec<u8>) -> Vec<u8> {
    let mut pad = int256(b.len() as i64);
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
    let pad = left_pad(b, false);
    pad
}

pub(crate) fn encode_byte_array(byte_array: Vec<Vec<u8>>, prepend_len: bool) -> Vec<u8> {
    let mut bytes = Vec::new();
    for b in byte_array.iter() {
        bytes.extend(b);
    }

    if prepend_len == true {
        let mut enc_bytes = int256(byte_array.len() as i64);
        enc_bytes.extend(bytes);
        return enc_bytes
    }
    bytes
}

pub(crate) fn encode_int_array(int_array: Vec<i64>, int_width: usize, prepend_len: bool) -> Vec<u8> {
    check_int_width(int_width);

    let mut bytes = Vec::new();
    for i in int_array.iter() {
        let enc_i = int256(*i as i64);
        bytes.extend(enc_i);
    }

    if prepend_len == true {
        let mut enc_bytes = int256(int_array.len() as i64);
        enc_bytes.extend(bytes);
        return enc_bytes
    }
    bytes
}

pub(crate) fn encode_uint_array(int_array: Vec<u64>, int_width: usize, prepend_len: bool) -> Vec<u8> {
    check_int_width(int_width);

    let mut bytes = Vec::new();
    for i in int_array.iter() {
        let enc_i = uint256(*i as u64);
        bytes.extend(enc_i);
    }

    if prepend_len == true {
        let mut enc_bytes = int256(int_array.len() as i64);
        enc_bytes.extend(bytes);
        return enc_bytes
    }
    bytes
}
