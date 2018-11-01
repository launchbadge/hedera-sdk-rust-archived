use crate::error::ErrorKind;
use crate::{
    proto::{self, ToProto},
    AccountId, Client, Duration, PublicKey, SecretKey, TransactionId,
};
use failure::Error;
use grpcio::Channel;
use protobuf::{Message, RepeatedField};

// Transaction
// ----------------------------------------------------------------------------

pub struct Transaction<T> {
    channel: Channel,
    operator: Option<AccountId>,
    node: Option<AccountId>,
    secrets: Vec<SecretKey>,
    memo: Option<String>,
    inner: Box<T>,
}

#[repr(C)]
pub struct TransactionResponse {
    id: TransactionId,
    precheck: u8,
}

impl<T> Transaction<T> {
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
            Some(cryptoCreateAccount(_)) => client.create_account(&tx),
            Some(cryptoTransfer(_)) => client.crypto_transfer(&tx),

            _ => unimplemented!(),
        };

        let response = response?;

        Ok(TransactionResponse {
            id: tx.take_body().take_transactionID().into(),
            precheck: response.get_nodeTransactionPrecheckCode() as u8,
        })
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

// TransactionCreateAccount
// ----------------------------------------------------------------------------

pub struct TransactionCreateAccount {
    key: Option<PublicKey>,
    initial_balance: u64,
}

impl Transaction<TransactionCreateAccount> {
    pub fn create_account(client: &Client) -> Self {
        Self {
            channel: client.channel.clone(),
            operator: None,
            node: None,
            memo: None,
            secrets: Vec::new(),
            inner: Box::new(TransactionCreateAccount {
                key: None,
                initial_balance: 0,
            }),
        }
    }

    pub fn key(&mut self, key: PublicKey) -> &mut Self {
        self.inner.key = Some(key);
        self
    }

    pub fn initial_balance(&mut self, balance: u64) -> &mut Self {
        self.inner.initial_balance = balance;
        self
    }
}

impl ToProto<proto::Transaction::TransactionBody_oneof_data> for TransactionCreateAccount {
    fn to_proto(&self) -> Result<proto::Transaction::TransactionBody_oneof_data, Error> {
        let mut data = proto::CryptoCreate::CryptoCreateTransactionBody::new();
        data.set_initialBalance(self.initial_balance);

        let key = match self.key.as_ref() {
            Some(key) => key,
            None => Err(ErrorKind::MissingField("public_key"))?,
        };

        data.set_key(key.to_proto()?);
        data.set_autoRenewPeriod(Duration::new(2592000, 0).to_proto()?);

        Ok(proto::Transaction::TransactionBody_oneof_data::cryptoCreateAccount(data))
    }
}

// TransactionCryptoTransfer
// ----------------------------------------------------------------------------

pub struct TransactionCryptoTransfer {
    transfers: Vec<(AccountId, i64)>,
}

impl Transaction<TransactionCryptoTransfer> {
    pub fn crypto_transfer(client: &Client) -> Self {
        Self {
            channel: client.channel.clone(),
            operator: None,
            node: None,
            memo: None,
            secrets: Vec::new(),
            inner: Box::new(TransactionCryptoTransfer {
                transfers: Vec::new(),
            }),
        }
    }

    pub fn transfer(&mut self, id: AccountId, amount: i64) -> &mut Self {
        self.inner.transfers.push((id, amount));
        self
    }
}

impl ToProto<proto::Transaction::TransactionBody_oneof_data> for TransactionCryptoTransfer {
    fn to_proto(&self) -> Result<proto::Transaction::TransactionBody_oneof_data, Error> {
        let amounts: Result<Vec<proto::CryptoTransfer::AccountAmount>, Error> = self
            .transfers
            .iter()
            .map(|(id, amount)| {
                let mut pb = proto::CryptoTransfer::AccountAmount::new();
                pb.set_accountID(id.to_proto()?);
                pb.set_amount(*amount);
                Ok(pb)
            })
            .collect();

        let mut transfers = proto::CryptoTransfer::TransferList::new();
        transfers.set_accountAmounts(RepeatedField::from_vec(amounts?));

        let mut data = proto::CryptoTransfer::CryptoTransferTransactionBody::new();
        data.set_transfers(transfers);

        Ok(proto::Transaction::TransactionBody_oneof_data::cryptoTransfer(data))
    }
}
