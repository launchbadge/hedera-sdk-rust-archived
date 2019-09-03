use crate::{
    id::ContractId,
    proto::{self},
};
use failure::Error;
use hex;

#[derive(Debug, Clone)]
pub struct ContractLogInfo {
    pub contract_id: ContractId,
    pub bloom: Vec<u8>,
    pub topic: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl From<proto::ContractCallLocal::ContractLoginfo> for ContractLogInfo {
    fn from(mut log: proto::ContractCallLocal::ContractLoginfo) -> Self {
        Self {
            contract_id: log.take_contractID().into(),
            bloom: log.take_bloom(),
            topic: log.take_topic().into_vec(),
            data: log.take_data(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContractFunctionResult {
    pub contract_id: ContractId,
    pub contract_call_result: Vec<u8>,
    pub error_message: String,
    pub bloom: Vec<u8>,
    pub gas_used: u64,
    pub log_info: Vec<ContractLogInfo>,
}

impl ContractFunctionResult {
    fn get_byte_buffer(&self, offset: usize) -> u8 {
        self.contract_call_result[offset]
    }

    fn get_int_value_at(&self, value_offset: usize) -> isize {
        self.get_byte_buffer(value_offset + 28) as isize
    }

    fn get_int_256(&self, val_index: usize) -> Vec<u8> {
        self.contract_call_result[val_index * 32..(val_index + 1) * 32].to_vec()
    }

    fn get_array_length(&self, offset: usize) -> i64 {
        let ln_start = offset + 24;
        let ln_end = ln_start + 8;
        let mut ln_bytes: [u8; 8] = Default::default();
        ln_bytes.copy_from_slice(&self.contract_call_result[ln_start..ln_end]);
        let ln = unsafe { std::mem::transmute::<[u8; 8], u64>(ln_bytes) }.to_be();
        ln as i64
    }

    pub fn get_int(&self, val_index: usize) -> isize {
        self.get_int_value_at(val_index * 32)
    }

    pub fn get_long(&self, val_index: usize) -> i64 {
        self.get_int_value_at(val_index * 32 + 24) as i64
    }

    pub fn get_bytes(&self, val_index: usize) -> Vec<u8> {
        let offset = self.get_int(val_index) as usize;
        let l = self.get_int_value_at(offset as usize) as usize;
        self.contract_call_result[offset + 32..offset + 32 + l].to_vec()
    }

    pub fn get_byte_array(&self, val_index: usize) -> Vec<Vec<u8>> {
        let offset = (val_index * 32) + 32;
        let start = offset + 32;
        let ln = self.get_array_length(offset) as usize;
        let mut b = Vec::new();
        for i in 0..ln {
            let b_offset = (i * 32) + start;
            let chunk = self.contract_call_result[b_offset..b_offset + 32].to_vec();
            b.push(chunk);
        }
        b
    }

    pub fn get_string(&self, val_index: usize) -> Result<String, Error> {
        let b = self.get_bytes(val_index);
        let s = String::from_utf8(b)?;
        Ok(s)
    }

    pub fn get_bool(&self, val_index: usize) -> bool {
        self.get_byte_buffer(val_index * 32 + 31) != 0
    }

    pub fn get_address(&self, val_index: usize) -> Vec<u8> {
        let offset = val_index * 32;
        self.contract_call_result[offset + 12..offset + 32].to_vec()
    }

    pub fn get_address_array(&self, val_index: usize) -> Vec<String> {
        let offset = (val_index * 32) + 32;
        let start = offset + 32;
        let ln = self.get_array_length(offset) as usize;
        let mut addrs = Vec::new();
        for i in 0..ln {
            let addr_offset = (i * 32) + start;
            let chunk = self.contract_call_result[addr_offset + 12..addr_offset + 32].to_vec();
            addrs.push(hex::encode(chunk))
        }
        addrs
    }
}

impl From<proto::ContractCallLocal::ContractFunctionResult> for ContractFunctionResult {
    fn from(mut result: proto::ContractCallLocal::ContractFunctionResult) -> Self {
        Self {
            contract_id: result.take_contractID().into(),
            contract_call_result: result.take_contractCallResult(),
            error_message: result.take_errorMessage(),
            bloom: result.take_bloom(),
            gas_used: result.get_gasUsed(),
            log_info: result.take_logInfo().into_iter().map(Into::into).collect(),
        }
    }
}