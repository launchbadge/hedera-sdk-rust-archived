use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryResponse, ToQueryProto},
    Client, ContractId, function_result::ContractFunctionResult
};
use failure::Error;

pub struct QueryContractCall {
    contract_id: ContractId,
    gas: i64,
    function_parameters: Vec<u8>,
    max_result_size: i64
}

impl QueryContractCall {
    pub fn new(client: &Client, contract_id: ContractId, gas: i64,
    function_parameters: Vec<u8>, max_result_size: i64) -> Query<Self> {
        Query::new(
            client,
            Self {
            contract_id,
            gas,
            function_parameters,
            max_result_size
        })
    }
}

impl QueryResponse for QueryContractCall {
    type Response = ContractFunctionResult;

    fn get(mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        Ok(response.take_contractCallLocal().take_functionResult().into())
    }
}

impl ToQueryProto for QueryContractCall {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::ContractCallLocal::ContractCallLocalQuery::new();
        query.set_header(header);
        query.set_contractID(self.contract_id.to_proto()?);
        query.set_gas(self.gas);
        query.set_functionParameters(self.function_parameters.clone());
        query.set_maxResultSize(self.max_result_size);

        Ok(Query_oneof_query::contractCallLocal(query))
    }
}