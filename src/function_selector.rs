use sha3::{Digest, Keccak256};

pub const SELECTOR_LEN: i8 = 4;
pub const SELECTOR_LEN_HEX: i8 = 8;

#[derive(Clone)]
pub struct FunctionSelector {
    pub needs_comma: bool,
    pub finished: Vec<u8>,
    pub complete: bool
}

impl FunctionSelector {
    pub fn new(function: String) -> Self {
        let p = "(".to_string().as_bytes().to_vec();
        let mut f = function.as_bytes().to_vec();
        f.extend(p);
        Self {
            needs_comma: false,
            finished: f,
            complete: false
        }
    }

    pub(crate) fn add_param_type(&mut self, param_type: String) {
        if self.needs_comma == true {
            self.finished.extend(",".as_bytes().to_vec())
        }
        self.finished.extend(param_type.as_bytes().to_vec());
        self.needs_comma = true;
        ()
    }

    pub(crate) fn finish_intermediate(&self) -> Vec<u8> {
        let mut f = self.finished.clone();
        if self.complete != true {
            f.extend(")".as_bytes().to_vec());
        }
        let mut hasher = Keccak256::default();
        hasher.input(&f);
        let out = hasher.result().to_vec();
        out
    }

    pub(crate) fn finish(&mut self) -> Vec<u8> {
        if self.complete != true {
            self.finished.extend(")".as_bytes().to_vec());
            self.complete = true;
        }
        let mut hasher = Keccak256::default();
        hasher.input(&self.finished);
        let out = hasher.result().to_vec();
        out
    }
}
