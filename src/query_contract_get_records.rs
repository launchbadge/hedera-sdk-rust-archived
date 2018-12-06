use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto },
    query::{Query, QueryInner},
    Client, ErrorKind, ContractId, PreCheckCode,
    transaction::TransactionRecord
};
use failure::Error;
use std::convert::{TryFrom, TryInto};

pub struct QueryContractGetRecordResponse {
    contract: ContractId,
    records: Vec<TransactionRecord>,
}

pub struct QueryContractGetRecord {
    contract: ContractId
}

impl QueryContractGetRecord {
    pub fn new(client: &Client, contract: ContractId) -> Query<QueryContractGetRecordResponse> {
        Query::new(client, Self { contract })
    }
}

impl TryFrom<proto::ContractGetRecords::ContractGetRecordsResponse> for QueryContractGetRecordResponse {
    type Error = Error;

    fn try_from(mut response: proto::ContractGetRecords::ContractGetRecordsResponse) -> Result<Self, Error> {
        // let records = TransactionRecord::try_from(&mut response.take_records()).unwrap();
        Ok(Self {
            contract: ContractId::from(response.take_contractID()),
            records: response.take_records().into_iter().map(TryInto::try_into).collect::<Result<Vec<_>, _>>().unwrap(),
        })
    }
}

impl QueryInner for QueryContractGetRecord {
    type Response = QueryContractGetRecordResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_contractGetRecordsResponse();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.try_into()?),
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