use crate::{
    id::ContractId,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryResponse, ToQueryProto},
    Client, TransactionId, TransactionRecord,
};
use failure::Error;
use try_from::TryInto;

// TODO: Move to ContractCallLocalQuery once it exists
#[derive(Debug, Clone)]
pub struct ContractLogInfo {
    pub bloom: Vec<u8>,
    pub topic: Vec<Vec<u8>>,
    pub data: Vec<u8>,
}

impl From<proto::ContractCallLocal::ContractLoginfo> for ContractLogInfo {
    fn from(mut log: proto::ContractCallLocal::ContractLoginfo) -> Self {
        Self {
            bloom: log.take_bloom(),
            topic: log.take_topic().into_vec(),
            data: log.take_data(),
        }
    }
}

// TODO: Move to ContractCallLocalQuery once it exists
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

pub struct QueryTransactionGetRecord {
    transaction: TransactionId,
}

impl QueryTransactionGetRecord {
    pub fn new(client: &Client, transaction: TransactionId) -> Query<Self> {
        Query::new(client, Self { transaction })
    }
}

impl QueryResponse for QueryTransactionGetRecord {
    type Response = TransactionRecord;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        response
            .take_transactionGetRecord()
            .take_transactionRecord()
            .try_into()
    }
}

impl ToQueryProto for QueryTransactionGetRecord {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::TransactionGetRecord::TransactionGetRecordQuery::new();
        query.set_header(header);
        query.set_transactionID(self.transaction.to_proto()?);

        Ok(Query_oneof_query::transactionGetRecord(query))
    }
}
