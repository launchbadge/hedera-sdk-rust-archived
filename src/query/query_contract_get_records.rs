use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryResponse, ToQueryProto},
    Client, ContractId, TransactionRecord,
};
use failure::Error;
use try_from::TryInto;

pub struct QueryContractGetRecords {
    contract: ContractId,
}

impl QueryContractGetRecords {
    pub fn new(client: &Client, contract: ContractId) -> Query<Self> {
        Query::new(client, Self { contract })
    }
}

impl QueryResponse for QueryContractGetRecords {
    type Response = Vec<TransactionRecord>;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        response.take_contractGetRecordsResponse().try_into()
    }
}

impl ToQueryProto for QueryContractGetRecords {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetRecords::ContractGetRecordsQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract.to_proto()?);

        Ok(Query_oneof_query::ContractGetRecords(query))
    }
}
