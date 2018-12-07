use failure::{err_msg, Error};
use protobuf::RepeatedField;
use try_from::TryInto;

use crate::{
    claim::Claim,
    crypto::PublicKey,
    id::{AccountId, ContractId, FileId},
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    query::{Query, QueryInner},
    Client,
};

pub enum Entity {
    Account(AccountId),
    Claim(Claim),
    File(FileId),
    Contract(ContractId),
}

fn try_into_entities(ids: RepeatedField<proto::GetByKey::EntityID>) -> Result<Vec<Entity>, Error> {
    use self::proto::GetByKey::EntityID_oneof_entity::*;

    ids.into_iter()
        .map(|id| match id.entity {
            Some(accountID(account_id)) => Ok(Entity::Account(account_id.into())),
            Some(claim(c)) => Ok(Entity::Claim(c.try_into()?)),
            Some(fileID(file_id)) => Ok(Entity::File(file_id.into())),
            Some(contractID(contract_id)) => Ok(Entity::Contract(contract_id.into())),
            None => Err(err_msg("empty entity id?")),
        })
        .collect::<Result<Vec<Entity>, Error>>()
}

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
        let mut response = response.take_getByKey();
        let header = response.take_header();

        try_precheck!(header).and_then(move |_| try_into_entities(response.take_entities()))
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::GetByKey::GetByKeyQuery::new();
        query.set_header(header);
        query.set_key(self.key.to_proto()?);

        Ok(Query_oneof_query::getByKey(query))
    }
}
