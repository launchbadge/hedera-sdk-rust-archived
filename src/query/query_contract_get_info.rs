use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryResponse, ToQueryProto},
    Client, ContractId, ContractInfo,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryContractGetInfo {
    contract: ContractId,
}

impl QueryContractGetInfo {
    pub fn new(client: &Client, contract: ContractId) -> Query<Self> {
        Query::new(client, Self { contract })
    }
}

impl QueryResponse for QueryContractGetInfo {
    type Response = ContractInfo;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        response
            .take_contractGetInfo()
            .take_contractInfo()
            .try_into()
    }
}

impl ToQueryProto for QueryContractGetInfo {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetInfo::ContractGetInfoQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract.to_proto()?);

        Ok(Query_oneof_query::contractGetInfo(query))
    }
}
