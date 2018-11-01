use crate::{
    proto::{self, Query::Query_oneof_query, QueryHeader::QueryHeader, ToProto},
    AccountId, Client, TransactionId,
};
use failure::Error;
use grpcio::Channel;

// ResponseType
// ----------------------------------------------------------------------------

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ResponseKind {
    Answer = 0,
    // unsupported: AnswerStateProof = 1,
    CostAnswer = 2,
    CostAnswerStateProof = 3,
}

impl Default for ResponseKind {
    fn default() -> Self {
        ResponseKind::Answer
    }
}

impl From<ResponseKind> for proto::QueryHeader::ResponseType {
    fn from(type_: ResponseKind) -> Self {
        use self::proto::QueryHeader::ResponseType::*;
        match type_ {
            ResponseKind::Answer => ANSWER_ONLY,
            // unsupported: ResponseKind::AnswerStateProof => ANSWER_STATE_PROOF,
            ResponseKind::CostAnswer => COST_ANSWER,
            ResponseKind::CostAnswerStateProof => COST_ANSWER_STATE_PROOF,
        }
    }
}

// ToQueryProto
// ----------------------------------------------------------------------------

pub trait ToQueryProto {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error>;
}

// Query
// ----------------------------------------------------------------------------

pub struct Query<T> {
    channel: Channel,
    kind: ResponseKind,
    // TODO: payment: Transaction,
    inner: T,
}

#[repr(C)]
pub struct QueryResponse<T> {
    pub precheck: u8,
    pub kind: ResponseKind,
    pub cost: u64,
    pub answer: T,
}

impl<T: ToQueryProto> Query<T> {
    pub fn kind(&mut self, kind: ResponseKind) -> &mut Self {
        self.kind = kind;
        self
    }

    fn to_proto(&self) -> Result<proto::Query::Query, Error> {
        let mut header = proto::QueryHeader::QueryHeader::new();
        header.set_responseType(self.kind.into());

        let mut query = proto::Query::Query::new();
        query.query = Some(self.inner.to_query_proto(header)?);

        Ok(query)
    }
}

// QueryGetAccountBalance
// ----------------------------------------------------------------------------

pub struct QueryGetAccountBalance {
    account: AccountId,
}

#[repr(C)]
pub struct QueryGetAccountBalanceAnswer {
    balance: u64,
}

impl Query<QueryGetAccountBalance> {
    pub fn new(ch: &Client, account: AccountId) -> Self {
        Self {
            channel: ch.channel.clone(),
            kind: ResponseKind::default(),
            inner: QueryGetAccountBalance { account },
        }
    }

    pub fn send(self) -> Result<QueryResponse<QueryGetAccountBalanceAnswer>, Error> {
        let query = self.to_proto()?;
        let client = proto::CryptoService_grpc::CryptoServiceClient::new(self.channel);

        let mut response = client.crypto_get_balance(&query)?;
        let mut response = response.take_cryptogetAccountBalance();
        let header = response.take_header();

        Ok(QueryResponse {
            precheck: header.get_nodeTransactionPrecheckCode() as u8,
            kind: self.kind,
            cost: header.get_cost(),
            answer: QueryGetAccountBalanceAnswer {
                balance: response.get_balance(),
            },
        })
    }
}

impl ToQueryProto for QueryGetAccountBalance {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::CryptoGetAccountBalance::CryptoGetAccountBalanceQuery::new();
        query.set_header(header);
        query.set_accountID(self.account.to_proto()?);

        Ok(Query_oneof_query::cryptogetAccountBalance(query))
    }
}

// QueryGetTransactionReceipt
// ----------------------------------------------------------------------------

pub struct QueryGetTransactionReceipt {
    transaction_id: TransactionId,
}

#[repr(C)]
pub struct QueryGetTransactionReceiptAnswer {
    status: u8,
    account_id: Option<Box<AccountId>>,
    // unsupported: contract_id: Option<Box<ContractId>>,
    // unsupported: file_id: Option<Box<FileId>>,
}

impl Query<QueryGetTransactionReceipt> {
    pub fn new(ch: &Client, transaction_id: TransactionId) -> Self {
        Self {
            channel: ch.channel.clone(),
            kind: ResponseKind::default(),
            inner: QueryGetTransactionReceipt { transaction_id },
        }
    }

    pub fn send(self) -> Result<QueryResponse<QueryGetTransactionReceiptAnswer>, Error> {
        let query = self.to_proto()?;
        let client = proto::CryptoService_grpc::CryptoServiceClient::new(self.channel);

        let mut response = client.get_transaction_receipts(&query)?;
        let mut response = response.take_transactionGetReceipt();
        let header = response.take_header();
        let mut receipt = response.take_receipt();

        let account_id = if receipt.has_accountID() {
            Some(Box::new(receipt.take_accountID().into()))
        } else {
            None
        };

        Ok(QueryResponse {
            precheck: header.get_nodeTransactionPrecheckCode() as u8,
            kind: self.kind,
            cost: header.get_cost(),
            answer: QueryGetTransactionReceiptAnswer {
                status: receipt.get_status() as u8,
                account_id,
            },
        })
    }
}

impl ToQueryProto for QueryGetTransactionReceipt {
    fn to_query_proto(&self, header: QueryHeader) -> Result<Query_oneof_query, Error> {
        let mut query = proto::TransactionGetReceipt::TransactionGetReceiptQuery::new();
        query.set_header(header);
        query.set_transactionID(self.transaction_id.to_proto()?);

        Ok(Query_oneof_query::transactionGetReceipt(query))
    }
}
