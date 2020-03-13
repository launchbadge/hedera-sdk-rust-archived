use crate::{ crypto::PublicKey, proto, AccountId, Claim, ContractId, FileId };
use chrono::{DateTime, Utc};
use failure::Error;
use std::time::Duration;
use try_from::{TryFrom, TryInto};

#[derive(Debug)]
pub struct AccountInfo {
    pub account_id: AccountId,
    pub contract_account_id: String,
    pub deleted: bool,
    pub proxy_account_id: Option<AccountId>,
    pub proxy_received: i64,
    pub key: PublicKey,
    pub balance: u64,
    pub generate_send_record_threshold: u64,
    pub generate_receive_record_threshold: u64,
    pub receiver_signature_required: bool,
    pub expiration_time: DateTime<Utc>,
    pub auto_renew_period: Duration,
    pub claims: Vec<Claim>,
}

impl TryFrom<proto::CryptoGetInfo::CryptoGetInfoResponse_AccountInfo> for AccountInfo {
    type Err = Error;

    fn try_from(
        mut info: proto::CryptoGetInfo::CryptoGetInfoResponse_AccountInfo,
    ) -> Result<Self, Error> {
        Ok(Self {
            account_id: info.take_accountID().into(),
            contract_account_id: info.take_contractAccountID(),
            deleted: info.get_deleted(),
            proxy_account_id: if info.has_proxyAccountID() {
                Some(info.take_proxyAccountID().into())
            } else {
                None
            },
            proxy_received: info.get_proxyReceived(),
            key: info.take_key().try_into()?,
            balance: info.get_balance(),
            generate_send_record_threshold: info.get_generateSendRecordThreshold(),
            generate_receive_record_threshold: info.get_generateReceiveRecordThreshold(),
            receiver_signature_required: info.get_receiverSigRequired(),
            expiration_time: info.take_expirationTime().into(),
            auto_renew_period: info.take_autoRenewPeriod().try_into()?,
            claims: info
                .take_claims()
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug)]
pub struct ContractInfo {
    pub contract_id: ContractId,
    pub account_id: AccountId,
    pub contract_account_id: String,
    pub admin_key: Option<PublicKey>,
    pub expiration_time: DateTime<Utc>,
    pub auto_renew_period: Duration,
    pub storage: i64,
    pub memo: String,
    pub balance: u64
}

impl TryFrom<proto::ContractGetInfo::ContractGetInfoResponse_ContractInfo> for ContractInfo {
    type Err = Error;

    fn try_from(
        mut info: proto::ContractGetInfo::ContractGetInfoResponse_ContractInfo,
    ) -> Result<Self, Error> {
        let admin_key = if info.has_adminKey() {
            Some(info.take_adminKey().try_into()?)
        } else {
            None
        };

        Ok(Self {
            contract_id: info.take_contractID().into(),
            account_id: info.take_accountID().into(),
            contract_account_id: info.take_contractAccountID(),
            admin_key,
            expiration_time: info.take_expirationTime().into(),
            auto_renew_period: info.take_autoRenewPeriod().try_into()?,
            storage: info.get_storage(),
            memo: info.get_memo(),
            balance: info.get_balance()
        })
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub file_id: FileId,
    pub size: i64,
    pub expiration_time: DateTime<Utc>,
    pub deleted: bool,
    pub keys: Vec<PublicKey>,
}

impl TryFrom<proto::FileGetInfo::FileGetInfoResponse_FileInfo> for FileInfo {
    type Err = Error;

    fn try_from(mut info: proto::FileGetInfo::FileGetInfoResponse_FileInfo) -> Result<Self, Error> {
        Ok(Self {
            file_id: info.take_fileID().into(),
            size: info.get_size(),
            expiration_time: info.take_expirationTime().into(),
            deleted: info.get_deleted(),
            keys: info
                .take_keys()
                .take_keys()
                .into_iter()
                .map(|k| k.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

#[derive(Debug)]
pub struct ConsensusTopicInfo {
    pub memo: String,
    pub running_hash: Vec<u8>,
    pub sequence_number: u64,
    pub expiration_time: DateTime<Utc>,
    pub admin_key: Option<PublicKey>,
    pub submit_key: Option<PublicKey>,
    pub auto_renew_period: Duration,
    pub auto_renew_account: Option<AccountId>
}

impl TryFrom<proto::ConsensusTopicInfo::ConsensusTopicInfo> for ConsensusTopicInfo {
    type Err = Error;

    fn try_from(mut info: proto::ConsensusTopicInfo::ConsensusTopicInfo) -> Result<Self, Error> {
        let _admin_key = if info.has_adminKey() {
            Some(info.take_adminKey().try_into()?)
        } else {
            None
        };

        let _submit_key = if info.has_submitKey() {
            Some(info.take_submitKey().try_into()?)
        } else {
            None
        };

        let _auto_renew_account = if info.has_autoRenewAccount() {
            Some(info.take_autoRenewAccount().try_into()?)
        } else {
            None
        };

        Ok(Self {
            memo: info.get_memo(),
            running_hash: info.take_runningHash(),
            sequence_number: info.get_sequenceNumber(),
            expiration_time: info.take_expirationTime().into(),
            admin_key: _admin_key,
            submit_key: _submit_key,
            auto_renew_period: info.take_autoRenewPeriod().try_into()?,
            auto_renew_account: _auto_renew_account
        })
    }
}
