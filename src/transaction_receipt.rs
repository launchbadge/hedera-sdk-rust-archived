use crate::{proto, AccountId, ContractId, FileId, Status};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TransactionReceipt {
    pub status: Status,
    pub account_id: Option<Box<AccountId>>,
    pub contract_id: Option<Box<ContractId>>,
    pub file_id: Option<Box<FileId>>,
}

impl std::fmt::Display for TransactionReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TX Receipt\n\tStatus: {:#?}\n\tAccount: {:#?}\n\tContract: {:#?}\n\tFile: {:#?}",
               self.status, self.account_id, self.contract_id, self.file_id)
    }
}

impl From<proto::TransactionReceipt::TransactionReceipt> for TransactionReceipt {
    fn from(mut receipt: proto::TransactionReceipt::TransactionReceipt) -> Self {
        let account_id = if receipt.has_accountID() {
            Some(Box::new(receipt.take_accountID().into()))
        } else {
            None
        };

        let file_id = if receipt.has_fileID() {
            Some(Box::new(receipt.take_fileID().into()))
        } else {
            None
        };

        let contract_id = if receipt.has_contractID() {
            Some(Box::new(receipt.take_contractID().into()))
        } else {
            None
        };

        Self {
            status: receipt.get_status().into(),
            account_id,
            contract_id,
            file_id,
        }
    }
}
