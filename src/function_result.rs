use crate::{
    id::ContractId,
    proto::{self},
};

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