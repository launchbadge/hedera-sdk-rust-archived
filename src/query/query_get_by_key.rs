use crate::{
    crypto::PublicKey,
    id::{AccountId, FileId, ContractId},
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto, GetByKey::EntityID_oneof_entity::*},
    query::{Query, QueryInner},
    claim::Claim,
    Client, ErrorKind, PreCheckCode,
};
use failure::Error;
use std::convert::{TryFrom, TryInto};
use protobuf::RepeatedField;

pub enum Entity {
    Account(AccountId),
    Claim(Claim),
    File(FileId),
    Contract(ContractId),
}

pub struct QueryGetByKeyResponse {
    pub entities: Vec<Entity>,
}

impl TryFrom<RepeatedField<proto::GetByKey::EntityID>> for QueryGetByKeyResponse {
    type Error = Error;

    fn try_from(response: RepeatedField<proto::GetByKey::EntityID>) -> Result<Self, Error> {
        Ok(Self {
            entities: response
                .into_iter()
                .filter_map(|id| id.entity)
                .map(|entity| match entity {
                    accountID(account_id) => {
                        Ok(Entity::Account(account_id.try_into()?))
                    },
                    claim(claim_id) => {
                        Ok(Entity::Claim(claim_id.try_into()?))
                    },
                    fileID(file_id) => {
                        Ok(Entity::File(file_id.try_into()?))
                    },
                    contractID(contract_id) => {
                        Ok(Entity::Contract(contract_id.try_into()?))
                    }
                })
                .collect::<Result<Vec<Entity>, Error>>()?
        })
    }
}

pub struct QueryGetByKey {
    key: PublicKey
}

impl QueryGetByKey {
    pub fn new(client: &Client, key: PublicKey) -> Query<QueryGetByKeyResponse> {
        Query::new(client, Self { key })
    }
}

impl QueryInner for QueryGetByKey {
    type Response = QueryGetByKeyResponse;

    fn get(&self, mut response: proto::Response::Response) -> Result<Self::Response, Error> {
        let mut response = response.take_getByKey();
        let header = response.take_header();

        match header.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(response.take_entities().try_into()?),
            code => Err(ErrorKind::PreCheck(code))?,
        }
    }

    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::GetByKey::GetByKeyQuery::new();
        query.set_header(header);
        query.set_key(self.key.to_proto()?);

        Ok(Query_oneof_query::getByKey(query))
    }
}
