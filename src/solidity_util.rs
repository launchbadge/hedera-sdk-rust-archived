// Utils for solidity interfaces
use hex;
use crate::id::{AccountId, ContractId, FileId};

const ADDRESS_LEN: i8 = 20;
const ADDRESS_LEN_HEX: i8 = 40;

pub(crate) fn check_address_len(addr: Vec<u8>) {
    if addr.len() != ADDRESS_LEN as usize {
        panic!("ILLEGAL ARGUMENT ERROR: Solidity addresses must be 20 bytes or 40 hex chars")
    }
}

pub(crate) fn decode_address(addr: String) -> Vec<u8> {
    if addr.chars().count() != ADDRESS_LEN_HEX as usize {
        panic!("ILLEGAL ARGUMENT ERROR: Solidity addresses must be 20 bytes or 40 hex chars")
    }

    let b = match hex::decode(addr) {
        Ok(a) => a.to_vec(),
        Err(e) => panic!("ILLEGAL ARGUMENT ERROR: failed to decode Solidity address as hex; \
        {:#?}", e)
    };
    b
}

pub(crate) fn address_for_entity(shard: i64, realm: i64, entity: i64) -> String {
    if shard < i32::min_value() as i64 || shard > i32::max_value() as i64 {
        panic!("ILLEGAL ARGUMENT ERROR: shard id should be within 32bit range")
    }
    let mut buf = Vec::new();

    let s = shard as i32;
    buf.extend(s.to_be_bytes().to_vec());
    buf.extend(realm.to_be_bytes().to_vec());
    buf.extend(entity.to_be_bytes().to_vec());

    let out = hex::encode(buf);
    out
}

pub(crate) fn entity_for_address(addr: String) -> (i64, i64, i64) {
    let decoded_address = decode_address(addr);

    let mut shard_chunk: [u8; 4] = Default::default();
    shard_chunk.copy_from_slice(&decoded_address[..4]);
    let mut realm_chunk: [u8; 8] = Default::default();
    realm_chunk.copy_from_slice(&decoded_address[4..12]);
    let mut entity_chunk: [u8; 8] = Default::default();
    entity_chunk.copy_from_slice(&decoded_address[12..]);

    let shard = unsafe { std::mem::transmute::<[u8; 4], u32>(shard_chunk) }.to_be();
    let realm = unsafe { std::mem::transmute::<[u8; 8], u64>(realm_chunk) }.to_be();
    let entity = unsafe { std::mem::transmute::<[u8; 8], u64>(entity_chunk) }.to_be();

    (shard as i64, realm as i64, entity as i64)
}

pub fn address_for_account(acct_id: AccountId) -> String {
    address_for_entity(acct_id.shard, acct_id.realm, acct_id.account)
}

pub fn address_for_contract(contract_id: ContractId) -> String {
    address_for_entity(contract_id.shard, contract_id.realm, contract_id.contract)
}

pub fn address_for_file(file_id: FileId) -> String {
    address_for_entity(file_id.shard, file_id.realm, file_id.file)
}

pub fn account_for_address(addr: String) -> AccountId {
    let (shard, realm, account) = entity_for_address(addr);
    AccountId{
        shard,
        realm,
        account
    }
}

pub fn account_for_contract(addr: String) -> ContractId {
    let (shard, realm, contract) = entity_for_address(addr);
    ContractId{
        shard,
        realm,
        contract
    }
}

pub fn account_for_file(addr: String) -> FileId {
    let (shard, realm, file) = entity_for_address(addr);
    FileId{
        shard,
        realm,
        file
    }
}
