use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client, ContractId, ContractInfo, ErrorKind, PreCheckCode,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryContractGetInfo {
    contract: ContractId,
}

impl QueryContractGetInfo {
    pub fn new(client: &Client, contract: ContractId) -> Query<ContractInfo> {
        Query::new(client, Self { contract })
    }
}

impl QueryInner for QueryContractGetInfo {
    type Response = ContractInfo;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_contractGetInfo();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => response.take_contractInfo().try_into(),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetInfo::ContractGetInfoQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract.to_proto()?);

        Ok(Query_oneof_query::contractGetInfo(query))
    }
}
