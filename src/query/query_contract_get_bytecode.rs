use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client, ContractId,
};
use failure::Error;

pub struct QueryContractGetBytecode {
    contract_id: ContractId,
}

impl QueryContractGetBytecode {
    pub fn new(client: &Client, contract_id: ContractId) -> Query<Vec<u8>> {
        Query::new(client, Self { contract_id })
    }
}

impl QueryInner for QueryContractGetBytecode {
    type Response = Vec<u8>;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_contractGetBytecodeResponse();
        let header = response.take_header();

        try_precheck!(header).map(move |_| response.take_bytecode())
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractGetBytecode::ContractGetBytecodeQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract_id.to_proto()?);

        Ok(Query_oneof_query::contractGetBytecode(query))
    }
}
