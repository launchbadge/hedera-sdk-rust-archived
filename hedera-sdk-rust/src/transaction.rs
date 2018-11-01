use crate::{
    proto::{self, ToProto},
    AccountId, Client, Duration, PublicKey, SecretKey, TransactionId,
};
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
    pub fn execute(self) -> TransactionResponse {
        use self::proto::Transaction::TransactionBody_oneof_data::*;

        let mut tx: proto::Transaction::Transaction = self.to_proto();
        let client = proto::CryptoService_grpc::CryptoServiceClient::new(self.channel);

        let response = match tx.get_body().data {
            Some(cryptoCreateAccount(_)) => client.create_account(&tx),
            Some(cryptoTransfer(_)) => client.crypto_transfer(&tx),

            _ => unimplemented!(),
        };

        // FIXME: Handle errors
        let response = response.unwrap();

        TransactionResponse {
            id: tx.take_body().take_transactionID().into(),
            precheck: response.get_nodeTransactionPrecheckCode() as u8,
        }
    }
}

impl<T> ToProto<proto::Transaction::Transaction> for Transaction<T>
where
    Transaction<T>: ToProto<proto::Transaction::TransactionBody>,
    T: ToProto<proto::Transaction::TransactionBody_oneof_data>,
{
    fn to_proto(&self) -> proto::Transaction::Transaction {
        let body = ToProto::<proto::Transaction::TransactionBody>::to_proto(self);

        // NOTE: This cannot fail.
        let body_bytes = body.write_to_bytes().unwrap();

        let signatures = self
            .secrets
            .iter()
            .map(|secret| secret.sign(&body_bytes).to_proto())
            .collect();

        let mut signature_list = proto::BasicTypes::SignatureList::new();
        signature_list.set_sigs(RepeatedField::from_vec(signatures));

        let mut tx = proto::Transaction::Transaction::new();
        tx.set_body(body);
        tx.set_sigs(signature_list);

        tx
    }
}

impl<T> ToProto<proto::Transaction::TransactionBody> for Transaction<T>
where
    T: ToProto<proto::Transaction::TransactionBody_oneof_data>,
{
    fn to_proto(&self) -> proto::Transaction::TransactionBody {
        // FIXME: Handle errors
        let tx_id = TransactionId::new(self.operator.unwrap());

        let mut body = proto::Transaction::TransactionBody::new();
        // FIXME: Handle errors
        body.set_nodeAccountID(self.node.unwrap().to_proto());
        body.set_transactionValidDuration(Duration::new(120, 0).to_proto());
        // FIXME: Figure out a good way to do fees
        body.set_transactionFee(10);
        body.set_generateRecord(false);
        body.set_transactionID(tx_id.to_proto());
        body.data = Some(self.inner.to_proto());
        body.set_memo(if let Some(memo) = &self.memo {
            memo.to_owned()
        } else {
            String::new()
        });

        body
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
    fn to_proto(&self) -> proto::Transaction::TransactionBody_oneof_data {
        let mut data = proto::CryptoCreate::CryptoCreateTransactionBody::new();
        data.set_initialBalance(self.initial_balance);
        // FIXME: Handle errors
        data.set_key(self.key.as_ref().unwrap().to_proto());
        data.set_autoRenewPeriod(Duration::new(2592000, 0).to_proto());

        proto::Transaction::TransactionBody_oneof_data::cryptoCreateAccount(data)
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
    fn to_proto(&self) -> proto::Transaction::TransactionBody_oneof_data {
        let amounts = self
            .transfers
            .iter()
            .map(|(id, amount)| {
                let mut pb = proto::CryptoTransfer::AccountAmount::new();
                pb.set_accountID(id.to_proto());
                pb.set_amount(*amount);
                pb
            })
            .collect();

        let mut transfers = proto::CryptoTransfer::TransferList::new();
        transfers.set_accountAmounts(RepeatedField::from_vec(amounts));

        let mut data = proto::CryptoTransfer::CryptoTransferTransactionBody::new();
        data.set_transfers(transfers);

        proto::Transaction::TransactionBody_oneof_data::cryptoTransfer(data)
    }
}
