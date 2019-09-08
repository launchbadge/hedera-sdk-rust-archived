use sha3::{Digest, Keccak256};
use std::string::ToString;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_selector() {
        let t1 = ("cdcd77c0".to_string(), "baz".to_string(),
                  "uint32".to_string(), "bool".to_string());
        let t2 = ("fce353f6".to_string(), "bar".to_string(), "bytes3[2]".to_string());
        let t3 = ("a5643bf2".to_string(), "sam".to_string(), "bytes".to_string(),
                  "bool".to_string(), "uint256[]".to_string());
        let t4 = ("8be65246".to_string(), "f".to_string(),
                  "uint256".to_string(), "uint32[]".to_string(), "bytes10".to_string(), "bytes".to_string());

        let mut fs: FunctionSelector;

        fs = FunctionSelector::new(t1.1);
        fs.add_param_type(t1.2);
        fs.add_param_type(t1.3);
        assert_eq!(hex::encode(fs.finish()[0..4].to_vec()), t1.0);


        fs = FunctionSelector::new(t2.1);
        fs.add_param_type(t2.2);
        assert_eq!(hex::encode(fs.finish()[0..4].to_vec()), t2.0);

        fs = FunctionSelector::new(t3.1);
        fs.add_param_type(t3.2);
        fs.add_param_type(t3.3);
        fs.add_param_type(t3.4);
        assert_eq!(hex::encode(fs.finish()[0..4].to_vec()), t3.0);

        fs = FunctionSelector::new(t4.1);
        fs.add_param_type(t4.2);
        fs.add_param_type(t4.3);
        fs.add_param_type(t4.4);
        fs.add_param_type(t4.5);
        assert_eq!(hex::encode(fs.finish()[0..4].to_vec()), t4.0);
    }
}
