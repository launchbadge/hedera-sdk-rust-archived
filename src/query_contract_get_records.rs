use protobuf::RepeatedField;
use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto, TransactionRecord::TransactionRecord},
    query::{Query, QueryInner},
    Client, ErrorKind, ContractId, PreCheckCode,

};
use failure::Error;

pub struct QueryContractGetRecordResponse {
    contract: ContractId,
    records: RepeatedField<TransactionRecord>,
}

pub struct QueryContractGetRecord {
    contract: ContractId
}

impl QueryContractGetRecord {
    pub fn new(client: &Client, contract: ContractId) -> Query<QueryContractGetRecordResponse> {
        Query::new(client, Self { contract })
    }
}

impl From<proto::ContractGetRecords::ContractGetRecordsResponse> for QueryContractGetRecordResponse {
    fn from(mut response: proto::ContractGetRecords::ContractGetRecordsResponse) -> Self {
        Self {
            contract: ContractId::from(response.take_contractID()),
            records: response.take_records()
        }
    }
}

impl QueryInner for QueryContractGetRecord {
    type Response = QueryContractGetRecordResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_contractGetRecordsResponse();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.into()),
            code => Err(ErrorKind::PreCheck(code))?
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetRecords::ContractGetRecordsQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract.to_proto()?);

        Ok(Query_oneof_query::ContractGetRecords(query))
    }
}