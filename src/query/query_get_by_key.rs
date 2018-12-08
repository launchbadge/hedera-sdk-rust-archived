use crate::{
    crypto::PublicKey,
    entity::try_into_entities,
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client, Entity,
};
use failure::Error;

pub struct QueryGetByKey {
    key: PublicKey,
}

impl QueryGetByKey {
    pub fn new(client: &Client, key: PublicKey) -> Query<Vec<Entity>> {
        Query::new(client, Self { key })
    }
}

impl QueryInner for QueryGetByKey {
    type Response = Vec<Entity>;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        try_into_entities(response.take_getByKey().take_entities())
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::GetByKey::GetByKeyQuery::new();
        query.set_header(header);
        query.set_key(self.key.to_proto()?);

        Ok(Query_oneof_query::getByKey(query))
    }
}
