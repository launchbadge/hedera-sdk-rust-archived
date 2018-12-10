use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{
        query::{QueryResponse, ToQueryProto},
        Query,
    },
    Client, ContractId,
};
use failure::Error;

pub struct QueryContractGetBytecode {
    contract_id: ContractId,
}

impl QueryContractGetBytecode {
    pub fn new(client: &Client, contract_id: ContractId) -> Query<Self> {
        Query::new(client, Self { contract_id })
    }
}

impl QueryResponse for QueryContractGetBytecode {
    type Response = Vec<u8>;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        Ok(response.take_contractGetBytecodeResponse().take_bytecode())
    }
}

impl ToQueryProto for QueryContractGetBytecode {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetBytecode::ContractGetBytecodeQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract_id.to_proto()?);

        Ok(Query_oneof_query::contractGetBytecode(query))
    }
}
