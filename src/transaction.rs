use crate::error::ErrorKind;
use crate::{
    proto::{self, ToProto},
    AccountId, Client, Duration, SecretKey, TransactionId,
};
use failure::Error;
use grpcio::Channel;
use protobuf::{Message, RepeatedField};

// Transaction Response
// ----------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum PreCheckCode {
    /// The transaction passed the pre-check.
    Ok = 0,

    /// The transaction had incorrect syntax or other errors.
    InvalidTransaction = 1,

    // The operator account or node account isn't a valid account number.
    InvalidAccount = 2,

    // The transaction fee is insufficient for this type of transaction.
    InsufficientFee = 3,

    // The operator account has insufficient crypto-currency to pay the transaction fee.
    InsufficientBalance = 4,

    /// This transaction ID is a duplicate of one that was submitted to this node or reached
    /// consensus in the last 180 seconds (receipt period).
    Duplicate = 5,

    /// Too many requests against the API
    Busy = 6,

    /// API is not supported
    NotSupported = 7,
}

impl From<proto::TransactionResponse::NodeTransactionPrecheckCode> for PreCheckCode {
    fn from(code: proto::TransactionResponse::NodeTransactionPrecheckCode) -> Self {
        use self::proto::TransactionResponse::NodeTransactionPrecheckCode::*;

        match code {
            OK => PreCheckCode::Ok,
            INVALID_TRANSACTION => PreCheckCode::InvalidTransaction,
            INVALID_ACCOUNT => PreCheckCode::InvalidAccount,
            INSUFFICIENT_FEE => PreCheckCode::InsufficientFee,
            INSUFFICIENT_BALANCE => PreCheckCode::InsufficientBalance,
            DUPLICATE => PreCheckCode::Duplicate,
            BUSY => PreCheckCode::Busy,
            NOT_SUPPORTED => PreCheckCode::NotSupported,
        }
    }
}

#[repr(C)]
pub struct TransactionResponse {
    pub id: TransactionId,
}

// Transaction
// ----------------------------------------------------------------------------

pub struct Transaction<T> {
    channel: Channel,
    operator: Option<AccountId>,
    node: Option<AccountId>,
    secrets: Vec<SecretKey>,
    memo: Option<String>,
    pub(crate) inner: Box<T>,
}

impl<T> Transaction<T> {
    pub(crate) fn new(client: &Client, inner: T) -> Self {
        Self {
            channel: client.channel.clone(),
            operator: None,
            node: None,
            memo: None,
            secrets: Vec::new(),
            inner: Box::new(inner),
        }
    }

    pub fn memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.memo = Some(memo.into());
        self
    }

    pub fn operator(&mut self, id: AccountId) -> &mut Self {
        self.operator = Some(id);
        self
    }

    pub fn node(&mut self, id: AccountId) -> &mut Self {
        self.node = Some(id);
        self
    }

    pub fn sign(&mut self, secret: SecretKey) -> &mut Self {
        self.secrets.push(secret);
        self
    }
}

impl<T> Transaction<T>
where
    Transaction<T>: ToProto<proto::Transaction::Transaction>,
    T: ToProto<proto::Transaction::TransactionBody_oneof_data>,
{
    pub fn execute(self) -> Result<TransactionResponse, Error> {
        use self::proto::Transaction::TransactionBody_oneof_data::*;

        let mut tx: proto::Transaction::Transaction = self.to_proto()?;
        let client = proto::CryptoService_grpc::CryptoServiceClient::new(self.channel);

        let response = match tx.get_body().data {
            Some(cryptoCreateAccount(_)) => client.create_account(&tx)?,
            Some(cryptoTransfer(_)) => client.crypto_transfer(&tx)?,

            _ => unimplemented!(),
        };

        match response.get_nodeTransactionPrecheckCode().into() {
            PreCheckCode::Ok => Ok(TransactionResponse {
                id: tx.take_body().take_transactionID().into(),
            }),

            code => Err(ErrorKind::PreCheck(code))?,
        }
    }
}

impl<T> ToProto<proto::Transaction::Transaction> for Transaction<T>
where
    Transaction<T>: ToProto<proto::Transaction::TransactionBody>,
    T: ToProto<proto::Transaction::TransactionBody_oneof_data>,
{
    fn to_proto(&self) -> Result<proto::Transaction::Transaction, Error> {
        let body = ToProto::<proto::Transaction::TransactionBody>::to_proto(self)?;

        // NOTE: This cannot fail.
        let body_bytes = body.write_to_bytes().unwrap();

        let signatures: Result<Vec<proto::BasicTypes::Signature>, Error> = self
            .secrets
            .iter()
            .map(|secret| Ok(secret.sign(&body_bytes).to_proto()?))
            .collect();

        let mut signature_list = proto::BasicTypes::SignatureList::new();
        signature_list.set_sigs(RepeatedField::from_vec(signatures?));

        let mut tx = proto::Transaction::Transaction::new();
        tx.set_body(body);
        tx.set_sigs(signature_list);

        Ok(tx)
    }
}

impl<T> ToProto<proto::Transaction::TransactionBody> for Transaction<T>
where
    T: ToProto<proto::Transaction::TransactionBody_oneof_data>,
{
    fn to_proto(&self) -> Result<proto::Transaction::TransactionBody, Error> {
        let account_id = self
            .operator
            .ok_or_else(|| ErrorKind::MissingField("account_id"))?;
        let tx_id = TransactionId::new(account_id);
        let mut body = proto::Transaction::TransactionBody::new();
        let node = self.node.ok_or_else(|| ErrorKind::MissingField("node"))?;

        body.set_nodeAccountID(node.to_proto()?);
        body.set_transactionValidDuration(Duration::new(120, 0).to_proto()?);
        // TODO: Figure out a good way to do fees
        body.set_transactionFee(10);
        body.set_generateRecord(false);
        body.set_transactionID(tx_id.to_proto()?);
        body.data = Some(self.inner.to_proto()?);
        body.set_memo(if let Some(memo) = &self.memo {
            memo.to_owned()
        } else {
            String::new()
        });

        Ok(body)
    }
}
