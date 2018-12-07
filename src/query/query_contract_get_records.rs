use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client, ContractId, ErrorKind, PreCheckCode, TransactionRecord,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryContractGetRecords {
    contract: ContractId,
}

impl QueryContractGetRecords {
    pub fn new(client: &Client, contract: ContractId) -> Query<Vec<TransactionRecord>> {
        Query::new(client, Self { contract })
    }
}

impl QueryInner for QueryContractGetRecords {
    type Response = Vec<TransactionRecord>;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_contractGetRecordsResponse();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => response.try_into(),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetRecords::ContractGetRecordsQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract.to_proto()?);

        Ok(Query_oneof_query::ContractGetRecords(query))
    }
}
