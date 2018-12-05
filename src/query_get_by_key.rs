use crate::{
    crypto::PublicKey,
    id::{Id, AccountId, FileId, ContractId},
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
                        let account_id: AccountId = account_id.try_into()?;

                        Ok(Entity::Account(account_id))
                    },
                    claim(claim_id) => {
                        let claim_id: Claim = claim_id.try_into()?;
                        Ok(Entity::Claim(claim_id))
                    },
                    fileID(file_id) => {
                        let file_id: FileId = file_id.try_into()?;
                        Ok(Entity::File(file_id))
                    },
                    contractID(contract_id) => {
                        let contract_id: ContractId = contract_id.try_into()?;
                        Ok(Entity::Contract(contract_id))
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
