use crate::argument::Argument;
use crate::function_selector::{FunctionSelector, SELECTOR_LEN, SELECTOR_LEN_HEX};
use crate::call_param_utils::*;
use crate::solidity_util::{check_address_len, decode_address};
use std::string::ToString;
use hex;

#[derive(Clone)]
pub struct CallParams {
    pub(crate) func_selector: Option<FunctionSelector>,
    pub(crate) args: Vec<Argument>
}

impl CallParams {
    pub fn new(func: Option<String>) -> Self {
        let fs = match func {
            Some(name) => Some(FunctionSelector::new(name)),
            None => None
        };
        let a = Vec::new();
        Self {
            func_selector: fs,
            args: a
        }
    }

    fn add_param_type(&mut self, param_type: String) {
        match self.func_selector.clone() {
            Some(mut fs) => {
                fs.add_param_type(param_type);
                self.func_selector = Some(fs);
                ()
            },
            None => (),
        };
    }

//    fn add_param_type(&mut self, param_type: String) {
//        self.func_selector = match self.func_selector.clone() {
//            Some(mut fs) => Some(fs.add_param_type(param_type)),
//            None => None,
//        };
//    }

    pub fn add_string(&mut self, param: String) {
        println!("[RUST] Add String Called: {:#?}", param);
        let enc_string = encode_string(param);
        let arg = Argument::new(enc_string, true);
        self.add_param_type("string".to_string());
        self.args.push(arg);
    }

    pub fn add_string_array(&mut self, param: Vec<String>) {
        let mut bytes = Vec::new();
        for s in param {
            let es = encode_string(s);
            bytes.push(es);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);

        self.add_param_type("string[]".to_string());
        self.args.push(arg);
    }

    pub fn add_fixed_string_array(&mut self, param: Vec<String>, fixed_len: usize) {
        check_fixed_array_len(&param[..], fixed_len);

        let mut bytes = Vec::new();
        for s in param {
            let es = encode_string(s);
            bytes.push(es);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);

        let param_type = format!("string[{:#?}]", fixed_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_bytes(&mut self, param: Vec<u8>) {
        let enc_bytes = encode_bytes(param);
        let arg = Argument::new(enc_bytes, true);
        self.add_param_type("bytes".to_string());
        self.args.push(arg);
    }

    pub fn add_fixed_bytes(&mut self, param: Vec<u8>, fixed_len: usize) {
        check_fixed_array_len(&param[..], fixed_len);

        if fixed_len > 32 {
            panic!("ILLEGAL ARGUMENT ERROR: bytesN cannot have a length greater than 32; \
            given length: {:#?}", fixed_len);
        }

        let enc_bytes = encode_fixed_bytes(param);
        let arg = Argument::new(enc_bytes, false);
        let param_type = format!("bytes{:#?}", fixed_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_byte_array(&mut self, param: Vec<Vec<u8>>) {
        let mut bytes = Vec::new();
        for b in param {
            let be = encode_bytes(b);
            bytes.push(be);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);
        self.add_param_type("bytes[]".to_string());
        self.args.push(arg);
    }

    pub fn add_fixed_byte_array(&mut self, param: Vec<Vec<u8>>, byte_len: usize) {
        for b in param.clone() {
            check_fixed_array_len(&b[..], byte_len);
        }

        if byte_len > 32 {
            panic!("ILLEGAL ARGUMENT ERROR: bytesN cannot have a length greater than 32; \
            given length: {:#?}", byte_len);
        }

        let mut bytes = Vec::new();
        for b in param {
            let be = encode_fixed_bytes(b);
            bytes.push(be);
        }

        let enc_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(enc_bytes, true);
        let param_type = format!("bytes{:#?}[]", byte_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_byte_fixed_array(&mut self, param: Vec<Vec<u8>>, fixed_len: usize) {
        check_fixed_array_len(&param[..], fixed_len);

        let mut bytes = Vec::new();
        for b in param {
            let be = encode_bytes(b);
            bytes.push(be);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);
        let param_type = format!("bytes[{:#?}]", fixed_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_fixed_byte_fixed_array(&mut self, param: Vec<Vec<u8>>, fixed_byte_len: usize,
                                      fixed_len: usize) {
        check_fixed_array_len(&param[..], fixed_len);
        for b in param.clone() {
            check_fixed_array_len(&b[..], fixed_byte_len);
        }

        let mut bytes = Vec::new();
        for b in param.clone() {
            let be = encode_fixed_bytes(b);
            bytes.push(be);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);
        let param_type = format!("bytes{:#?}[{:#?}]", fixed_byte_len, fixed_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_bool(&mut self, param: bool) {
        let mut val = 0i64;
        if param == true { val = 1i64; }

        let enc_bool = int256(val);
        let arg = Argument::new(enc_bool, false);
        self.add_param_type("bool".to_string());
        self.args.push(arg);
    }

    pub fn add_int(&mut self, param: i64, width: usize) {
        check_int_width(width);

        let enc_int = int256(param);
        let arg = Argument::new(enc_int, false);
        let param_type = format!("int{:#?}", width);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_int_array(&mut self, param: Vec<i64>, width: usize) {
        let arg_bytes = encode_int_array(param, width, true);
        let arg = Argument::new(arg_bytes, true);
        let param_type = format!("int{:#?}[]", width);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_fixed_int_array(&mut self, param: Vec<i64>, width: usize, fixed_len: usize) {
        check_fixed_array_len(&param[..], fixed_len);

        let arg_bytes = encode_int_array(param, width, true);
        let arg = Argument::new(arg_bytes, true);
        let param_type = format!("int{:#?}[{:#?}]", width, fixed_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_uint(&mut self, param: u64, width: usize) {
        check_int_width(width);

        let enc_uint = uint256(param);
        let arg = Argument::new(enc_uint, false);
        let param_type = format!("uint{:#?}", width);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_uint_array(&mut self, param: Vec<u64>, width: usize) {
        let arg_bytes = encode_uint_array(param, width, true);
        let arg = Argument::new(arg_bytes, true);
        let param_type = format!("uint{:#?}[]", width);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_fixed_uint_array(&mut self, param: Vec<u64>, width: usize, fixed_len: usize) {
        check_fixed_array_len(&param[..], fixed_len);

        let arg_bytes = encode_uint_array(param, width, true);
        let arg = Argument::new(arg_bytes, true);
        let param_type = format!("uint{:#?}[{:#?}]", width, fixed_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_address(&mut self, addr: Vec<u8>) {
        check_address_len(addr.clone());

        let enc_addr = left_pad(addr, false);
        let arg = Argument::new(enc_addr, false);
        self.add_param_type("address".to_string());
        self.args.push(arg);
    }

    pub fn add_address_string(&mut self, addr: String) {
        let a = decode_address(addr);
        self.add_address(a);
    }

    pub fn add_address_array(&mut self, addrs: Vec<Vec<u8>>) {
        let mut bytes = Vec::new();
        for a in addrs.clone() {
            check_address_len(a.clone());
            let ea = left_pad(a, false);
            bytes.push(ea);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);
        self.add_param_type("address[]".to_string());
        self.args.push(arg);
    }

    pub fn add_fixed_address_array(&mut self, addrs: Vec<Vec<u8>>, fixed_len: usize) {
        check_fixed_array_len(&addrs[..], fixed_len);

        let mut bytes = Vec::new();
        for a in addrs.clone() {
            check_address_len(a.clone());
            let ea = left_pad(a, false);
            bytes.push(ea);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);
        let param_type = format!("address[{:#?}]", fixed_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_address_string_array(&mut self, addrs: Vec<String>) {
        let mut bytes = Vec::new();
        for a in addrs.clone() {
            let da = decode_address(a);
            let ea = left_pad(da, false);
            bytes.push(ea);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);
        self.add_param_type("address[]".to_string());
        self.args.push(arg);
    }

    pub fn add_fixed_address_string_array(&mut self, addrs: Vec<String>, fixed_len: usize) {
        check_fixed_array_len(&addrs[..], fixed_len);

        let mut bytes = Vec::new();
        for a in addrs.clone() {
            let da = decode_address(a);
            let ea = left_pad(da, false);
            bytes.push(ea);
        }

        let arg_bytes = encode_byte_array(bytes, true);
        let arg = Argument::new(arg_bytes, true);
        let param_type = format!("address[{:#?}]", fixed_len);
        self.add_param_type(param_type);
        self.args.push(arg);
    }

    pub fn add_function(&mut self, addr: Vec<u8>, selector: Vec<u8>) {
        check_address_len(addr.clone());

        if selector.len() != SELECTOR_LEN as usize{
            panic!("ILLEGAL ARGUMENT ERROR: function selectors must be 4 bytes or 8 hex chars")
        }

        let mut bytes = Vec::new();
        bytes.extend(addr);
        bytes.extend(selector);

        let arg_bytes = right_pad(bytes);
        let arg = Argument::new(arg_bytes, false);
        self.add_param_type("function".to_string());
        self.args.push(arg);
    }

    pub fn add_function_string(&mut self, addr: String, selector: String) {
        if selector.chars().count() != SELECTOR_LEN_HEX as usize {
            panic!("ILLEGAL ARGUMENT ERROR: function selectors must be 4 bytes or 8 hex chars")
        }

        let s_bytes = match hex::decode(selector) {
            Ok(s) => s.to_vec(),
            Err(e) => panic!("failed to decode Solidity function selector as hex; {:#?}", e),
        };

        let a_bytes = match hex::decode(addr) {
            Ok(a) => a.to_vec(),
            Err(e) => panic!("failed to decode Solidity function selector as hex; {:#?}", e),
        };
        self.add_function(a_bytes, s_bytes);
    }

    pub fn add_function_fs(&mut self, addr: String, selector: FunctionSelector) {
        let a_bytes = match hex::decode(addr) {
            Ok(a) => a.to_vec(),
            Err(e) => panic!("failed to decode Solidity function selector as hex; {:#?}", e),
        };

        let fs = selector.finish_intermediate();
        self.add_function(a_bytes, fs[..4].to_vec())
    }

    pub fn assemble(&self) -> Vec<u8> {
        let mut dynamic_offset = self.args.len() * 32;
        let mut param_bytes = Vec::new();

        match self.func_selector.clone() {
            Some(fs) => {
                let f = fs.finish_intermediate();
                param_bytes.push(f[..4].to_vec());
                ()
            },
            None => (),
        };

        let mut dynamic_bytes = Vec::new();

        for arg in self.args.clone() {
            if arg.dynamic == true {
                let offset = int256(dynamic_offset as i64);
                param_bytes.push(offset);
                dynamic_bytes.push(arg.clone().value);
                dynamic_offset += arg.value.len();
            } else {
                param_bytes.push(arg.value);
            }
        }

        param_bytes.extend(dynamic_bytes);

        let mut out = Vec::new();
        for b in param_bytes {
            out.extend(b);
        }
        out
    }
}
