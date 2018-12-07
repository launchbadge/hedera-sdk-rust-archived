use crate::{proto, AccountId, Claim, ContractId, FileId};
use failure::Error;
use protobuf::RepeatedField;
use try_from::TryInto;

pub enum Entity {
    Account(AccountId),
    Claim(Claim),
    File(FileId),
    Contract(ContractId),
}

pub(crate) fn try_into_entities(
    ids: RepeatedField<proto::GetByKey::EntityID>,
) -> Result<Vec<Entity>, Error> {
    use self::proto::GetByKey::EntityID_oneof_entity::*;

    ids.into_iter()
        .map(|id| match id.entity {
            Some(accountID(account_id)) => Ok(Entity::Account(account_id.into())),
            Some(claim(c)) => Ok(Entity::Claim(c.try_into()?)),
            Some(fileID(file_id)) => Ok(Entity::File(file_id.into())),
            Some(contractID(contract_id)) => Ok(Entity::Contract(contract_id.into())),

            None => unreachable!(),
        })
        .collect::<Result<Vec<Entity>, Error>>()
}
