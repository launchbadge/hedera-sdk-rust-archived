use std::{sync::Arc, time::Duration};

use failure::{format_err, Error};
use itertools::Itertools;

use crate::{
    claim::Claim,
    crypto::SecretKey,
    id::{ContractId, FileId},
    proto::{
        CryptoService_grpc::CryptoServiceClient, FileService_grpc::FileServiceClient,
        SmartContractService_grpc::SmartContractServiceClient,
    },
    query::{
        Query, QueryCryptoGetAccountBalance, QueryCryptoGetClaim, QueryCryptoGetInfo,
        QueryFileGetContents, QueryFileGetInfo, QueryGetTransactionReceipt,
        QueryTransactionGetRecord,
    },
    transaction::{
        Transaction, TransactionContractCall, TransactionContractCreate, TransactionContractUpdate,
        TransactionCryptoCreate, TransactionCryptoDelete, TransactionCryptoDeleteClaim,
        TransactionCryptoTransfer, TransactionCryptoUpdate, TransactionFileAppend,
        TransactionFileCreate, TransactionFileDelete,
    },
    AccountId, AccountInfo, FileInfo, TransactionId, TransactionReceipt, TransactionRecord,
};

use grpc::ClientStub;

pub struct ClientBuilder<'a> {
    address: &'a str,
    node: Option<AccountId>,
    operator: Option<AccountId>,
    operator_secret: Option<SecretKey>,
}

pub struct Client {
    pub(crate) node: Option<AccountId>,
    pub(crate) operator: Option<AccountId>,
    pub(crate) operator_secret: Option<Arc<SecretKey>>,
    pub(crate) crypto: Arc<CryptoServiceClient>,
    pub(crate) file: Arc<FileServiceClient>,
    pub(crate) contract: Arc<SmartContractServiceClient>,
}

impl<'a> ClientBuilder<'a> {
    pub fn node(mut self, node: AccountId) -> Self {
        self.node = Some(node);
        self
    }

    pub fn operator(mut self, operator: AccountId, operator_secret: SecretKey) -> Self {
        self.operator = Some(operator);
        self.operator_secret = Some(operator_secret);

        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let mut client = Client::new(&self.address)?;

        if let Some(node) = self.node {
            client.set_node(node);
        }

        if let (Some(operator), Some(secret)) = (self.operator, self.operator_secret) {
            client.set_operator(operator, secret);
        }

        Ok(client)
    }
}

impl Client {
    pub fn builder(address: &str) -> ClientBuilder {
        ClientBuilder {
            address,
            node: None,
            operator: None,
            operator_secret: None,
        }
    }

    pub fn new(address: impl AsRef<str>) -> Result<Self, Error> {
        let address = address.as_ref();
        let (host, port) = address.split(':').next_tuple().ok_or_else(|| {
            format_err!("failed to parse 'host:port' from address: {:?}", address)
        })?;

        let port = port.parse()?;

        let inner = Arc::new(grpc::Client::new_plain(
            &host,
            port,
            grpc::ClientConf {
                http: httpbis::ClientConf {
                    no_delay: Some(true),
                    connection_timeout: Some(Duration::from_secs(5)),
                    ..httpbis::ClientConf::default()
                },
            },
        )?);

        let crypto = Arc::new(CryptoServiceClient::with_client(inner.clone()));
        let file = Arc::new(FileServiceClient::with_client(inner.clone()));
        let contract = Arc::new(SmartContractServiceClient::with_client(inner.clone()));

        // Default the node to what we know every testnet is on
        let node = if address.starts_with("testnet.") {
            Some(AccountId { shard: 0, realm: 0, account: 3 })
        } else {
            None
        };

        Ok(Self {
            node,
            operator: None,
            operator_secret: None,
            crypto,
            file,
            contract,
        })
    }

    #[inline]
    pub(crate) fn set_node(&mut self, node: AccountId) -> &mut Self {
        self.node = Some(node);

        self
    }

    #[inline]
    pub(crate) fn set_operator(
        &mut self,
        operator: AccountId,
        operator_secret: SecretKey,
    ) -> &mut Self {
        self.operator = Some(operator);
        self.operator_secret = Some(Arc::new(operator_secret));

        self
    }

    #[inline]
    pub fn transfer_crypto(&self) -> Transaction<TransactionCryptoTransfer> {
        TransactionCryptoTransfer::new(self)
    }

    /// Create a new account. After the account is created, the AccountID for it is in the
    /// receipt, or can be retrieved with a GetByKey query, or by asking for a Record of the
    /// transaction to be created, and retrieving that.
    #[inline]
    pub fn create_account(&self) -> Transaction<TransactionCryptoCreate> {
        TransactionCryptoCreate::new(self)
    }

    #[inline]
    pub fn account(&self, id: AccountId) -> PartialAccountMessage<'_> {
        PartialAccountMessage(self, id)
    }

    /// Start a new smart contract instance.
    #[inline]
    pub fn create_contract(&self) -> Transaction<TransactionContractCreate> {
        TransactionContractCreate::new(self)
    }

    #[inline]
    pub fn contract(&self, id: ContractId) -> PartialContractMessage<'_> {
        PartialContractMessage(self, id)
    }

    /// Create a new file.
    #[inline]
    pub fn create_file(&self) -> Transaction<TransactionFileCreate> {
        TransactionFileCreate::new(self)
    }

    #[inline]
    pub fn file(&self, id: FileId) -> PartialFileMessage<'_> {
        PartialFileMessage(self, id)
    }

    #[inline]
    pub fn transaction(&self, id: TransactionId) -> PartialTransactionMessage {
        PartialTransactionMessage(self, id)
    }
}

pub struct PartialAccountMessage<'a>(&'a Client, AccountId);

impl<'a> PartialAccountMessage<'a> {
    /// Get the balance of a crypto-currency account.
    #[inline]
    pub fn balance(self) -> Query<u64> {
        QueryCryptoGetAccountBalance::new(self.0, self.1)
    }

    /// Get all the information about an account, including the balance.
    #[inline]
    pub fn info(self) -> Query<AccountInfo> {
        QueryCryptoGetInfo::new(self.0, self.1)
    }

    /// Change properties for the given account. Any missing field is ignored (left unchanged).
    /// This transaction must be signed by the existing key for this account.
    #[inline]
    pub fn update(self) -> Transaction<TransactionCryptoUpdate> {
        TransactionCryptoUpdate::new(self.0, self.1)
    }

    /// Mark an account as deleted, moving all its current hbars to another account.
    /// It will remain in the ledger, marked as deleted, until it expires.
    #[inline]
    pub fn delete(self) -> Transaction<TransactionCryptoDelete> {
        TransactionCryptoDelete::new(self.0, self.1)
    }

    #[inline]
    pub fn claim(self, hash: impl Into<Vec<u8>>) -> PartialAccountClaimMessage<'a> {
        PartialAccountClaimMessage(self, hash.into())
    }
}

pub struct PartialAccountClaimMessage<'a>(PartialAccountMessage<'a>, Vec<u8>);

impl<'a> PartialAccountClaimMessage<'a> {
    /// Delete a claim hash that was attached to the given account.
    /// This transaction is valid if signed by all the keys used for transfers out of the account.
    #[inline]
    pub fn delete(self) -> Transaction<TransactionCryptoDeleteClaim> {
        TransactionCryptoDeleteClaim::new((self.0).0, (self.0).1, self.1)
    }

    #[inline]
    pub fn get(self) -> Query<Claim> {
        QueryCryptoGetClaim::new((self.0).0, (self.0).1, self.1)
    }
}

pub struct PartialFileMessage<'a>(&'a Client, FileId);

impl<'a> PartialFileMessage<'a> {
    #[inline]
    pub fn append(self, contents: Vec<u8>) -> Transaction<TransactionFileAppend> {
        TransactionFileAppend::new(self.0, self.1, contents)
    }

    #[inline]
    pub fn delete(self) -> Transaction<TransactionFileDelete> {
        TransactionFileDelete::new(self.0, self.1)
    }

    #[inline]
    pub fn info(self) -> Query<FileInfo> {
        QueryFileGetInfo::new(self.0, self.1)
    }

    #[inline]
    pub fn contents(self) -> Query<Vec<u8>> {
        QueryFileGetContents::new(self.0, self.1)
    }
}

pub struct PartialContractMessage<'a>(&'a Client, ContractId);

impl<'a> PartialContractMessage<'a> {
    #[inline]
    pub fn call(self) -> Transaction<TransactionContractCall> {
        TransactionContractCall::new(self.0, self.1)
    }

    #[inline]
    pub fn update(self) -> Transaction<TransactionContractUpdate> {
        TransactionContractUpdate::new(self.0, self.1)
    }
}

pub struct PartialTransactionMessage<'a>(&'a Client, TransactionId);

impl<'a> PartialTransactionMessage<'a> {
    /// Get the receipt of a transaction, given its transaction ID.
    ///
    /// Once a transaction reaches consensus, then information about whether it succeeded or
    /// failed will be available until the end of the receipt period.
    #[inline]
    pub fn receipt(self) -> Query<TransactionReceipt> {
        QueryGetTransactionReceipt::new(self.0, self.1)
    }

    /// Get the record for a transaction.
    ///
    /// If the transaction requested a record, then the record lasts for one hour, and a state
    /// proof is available for it.
    #[inline]
    pub fn record(self) -> Query<TransactionRecord> {
        QueryTransactionGetRecord::new(self.0, self.1)
    }
}
