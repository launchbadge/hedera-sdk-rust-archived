use crate::{
    crypto::SecretKey,
    error::ErrorKind,
    proto::{
        self,
        CryptoService_grpc::{CryptoService, CryptoServiceClient},
        FileService_grpc::{FileService, FileServiceClient},
        SmartContractService_grpc::{SmartContractService, SmartContractServiceClient},
        ToProto,
    },
    AccountId, Client, TransactionId,
};
use failure::Error;
use futures::{future, Future, TryFutureExt};
use protobuf::Message;
use query_interface::Object;
use std::{any::Any, marker::PhantomData, mem::swap, sync::Arc, time::Duration};
use tokio::await;
use tokio_async_await::compat::backward::Compat;

pub struct TransactionBuilder<T> {
    id: Option<TransactionId>,
    node: Option<AccountId>,
    memo: Option<String>,
    generate_record: bool,
    fee: u64,
    pub(crate) inner: Box<dyn Object>,
    phantom: PhantomData<T>,
}

pub struct TransactionRaw {
    bytes: Vec<u8>,
    pub(crate) tx: proto::Transaction::Transaction,
}

enum TransactionKind<T> {
    Empty,
    Err(Error),
    Builder(TransactionBuilder<T>),
    Raw(TransactionRaw),
}

impl<T> TransactionKind<T> {
    fn take(&mut self) -> Self {
        let mut temp = TransactionKind::Empty;
        swap(&mut temp, self);
        temp
    }
}

pub struct Transaction<T, S = TransactionBuilder<T>> {
    crypto_service: Arc<CryptoServiceClient>,
    file_service: Arc<FileServiceClient>,
    contract_service: Arc<SmartContractServiceClient>,
    secret: Option<Arc<dyn Fn() -> Result<SecretKey, Error> + Send + Sync>>,
    kind: TransactionKind<T>,
    phantom: PhantomData<S>,
}

impl<T: 'static> Transaction<T, TransactionBuilder<T>> {
    pub(crate) fn new(client: &Client, inner: T) -> Self
    where
        T: Object + ToProto<proto::Transaction::TransactionBody_oneof_data> + 'static,
    {
        Self {
            crypto_service: client.crypto.clone(),
            file_service: client.file.clone(),
            contract_service: client.contract.clone(),
            secret: client.operator_secret.clone(),
            kind: TransactionKind::Builder(TransactionBuilder {
                id: client.operator.map(TransactionId::new),
                node: client.node,
                memo: None,
                inner: Box::<T>::new(inner) as Box<dyn Object>,
                fee: 100_000,
                generate_record: false,
                phantom: PhantomData,
            }),
            phantom: PhantomData,
        }
    }

    pub fn memo(&mut self, memo: impl Into<String>) -> &mut Self {
        if let Some(state) = self.as_builder() {
            state.memo = Some(memo.into());
        }

        self
    }

    pub fn operator(&mut self, id: AccountId) -> &mut Self {
        // This resets any default operator we may have had
        self.secret = None;

        if let Some(state) = self.as_builder() {
            state.id = Some(TransactionId::new(id));
        }

        self
    }

    pub fn node(&mut self, id: AccountId) -> &mut Self {
        if let Some(state) = self.as_builder() {
            state.node = Some(id);
        }

        self
    }

    /// The fee the client pays, which is split between the network and the node.
    pub fn fee(&mut self, fee: u64) -> &mut Self {
        if let Some(state) = self.as_builder() {
            state.fee = fee;
        }

        self
    }

    /// Should a record of this transaction be generated?
    /// A receipt is always generated, but the record is optional.
    pub fn generate_record(&mut self, generate: bool) -> &mut Self {
        if let Some(state) = self.as_builder() {
            state.generate_record = generate;
        }

        self
    }

    pub fn sign(&mut self, secret: &SecretKey) -> &mut Transaction<T, TransactionRaw> {
        self.build().sign(secret)
    }

    pub fn execute_async(&mut self) -> impl Future<Output = Result<TransactionId, Error>> {
        self.build().execute_async()
    }

    pub fn execute(&mut self) -> Result<TransactionId, Error> {
        crate::RUNTIME
            .lock()
            .block_on(Compat::new(self.execute_async()))
    }

    #[inline]
    fn as_builder(&mut self) -> Option<&mut TransactionBuilder<T>> {
        match &mut self.kind {
            TransactionKind::Builder(ref mut state) => Some(state),

            TransactionKind::Raw(_) => {
                // should never be able to happen (in Rust)
                panic!("cannot edit a transaction after it has been signed")
            }

            TransactionKind::Err(_) => {
                // should never be able to happen (in Rust)
                None
            }

            _ => {
                // should never be able to happen (in Rust)
                panic!("transaction already executed")
            }
        }
    }

    #[inline]
    pub(crate) fn inner(&mut self) -> &mut T {
        // not possible to fail in safe rust
        match self
            .as_builder()
            .unwrap()
            .inner
            .query_mut::<dyn Any>()
            .and_then(|inner| inner.downcast_mut())
        {
            Some(inner) => inner,

            // not possible in safe rust to get here
            _ => unreachable!(),
        }
    }
}

impl<T: 'static> Transaction<T, TransactionRaw> {
    #[inline]
    pub(crate) fn as_raw(&mut self) -> Option<&mut TransactionRaw> {
        match &mut self.kind {
            TransactionKind::Builder(_) => {
                // not possible in safe rust
                unreachable!()
            }

            TransactionKind::Raw(ref mut state) => Some(state),

            TransactionKind::Err(_) => None,

            TransactionKind::Empty => {
                // should never be able to happen (in Rust)
                panic!("transaction already executed")
            }
        }
    }

    pub fn sign(&mut self, secret: &SecretKey) -> &mut Self {
        use self::proto::{
            BasicTypes::HederaFunctionality::*, Transaction::TransactionBody_oneof_data::*,
        };

        let has_secret = self.secret.is_some();
        if let Some(state) = self.as_raw() {
            // note: this cannot fail
            let mut signature = secret.sign(&state.bytes).to_proto().unwrap();

            // determine what kind of tx we have
            let kind = match state.tx.body.as_ref().unwrap().data {
                Some(fileCreate(_)) => Some(FileCreate),
                Some(fileAppend(_)) => Some(FileAppend),
                _ => None,
            };

            if !state.tx.has_sigs() {
                state.tx.set_sigs(proto::BasicTypes::SignatureList::new());
            }

            // note: this cannot fail
            let signatures = &mut state.tx.sigs.as_mut().unwrap().sigs;

            // signature #0 is for operator
            // signature #1 is for:
            //  - owner of _thing_ being created
            //  - # correspond to transfer

            if (has_secret || signatures.len() >= 1)
                && (kind == Some(FileCreate) || kind == Some(FileAppend))
            {
                // IF we are on signature #1 and we operating on a file or contract,
                // place the signature into a signature list

                let mut sig = proto::BasicTypes::Signature::new();
                sig.signature = signature.signature;

                let mut sigs = proto::BasicTypes::SignatureList::new();
                sigs.sigs.push(sig);

                signature = proto::BasicTypes::Signature::new();
                signature.set_signatureList(sigs);
            }

            signatures.push(signature);
        }

        self
    }

    pub fn execute(&mut self) -> Result<TransactionId, Error> {
        crate::RUNTIME
            .lock()
            .block_on(Compat::new(self.execute_async()))
    }

    pub fn execute_async(&mut self) -> impl Future<Output = Result<TransactionId, Error>> {
        use self::proto::Transaction::TransactionBody_oneof_data::*;

        let crypto = self.crypto_service.clone();
        let file = self.file_service.clone();
        let contract = self.contract_service.clone();

        future::ready(self.take_raw()).and_then(async move |state| {
            let mut tx = state.tx;

            // note: cannot fail
            let id = tx
                .body
                .as_ref()
                .unwrap()
                .transactionID
                .as_ref()
                .unwrap()
                .clone();

            log::trace!(target: "hedera::transaction", "sent: {:#?}", tx);

            let o = grpc::RequestOptions::default();
            let response = match tx.mut_body().data {
                Some(cryptoCreateAccount(_)) => crypto.create_account(o, tx),
                Some(cryptoTransfer(_)) => crypto.crypto_transfer(o, tx),
                Some(cryptoDeleteClaim(_)) => crypto.delete_claim(o, tx),
                Some(cryptoDelete(_)) => crypto.crypto_delete(o, tx),
                Some(fileCreate(_)) => file.create_file(o, tx),
                Some(fileAppend(_)) => file.append_content(o, tx),
                Some(contractCreateInstance(_)) => contract.create_contract(o, tx),

                _ => unimplemented!(),
            };

            let response = await!(response.drop_metadata())?;
            log::trace!("recv: {:#?}", response);

            try_precheck!(response).map(|_| id.into())
        })
    }
}

impl<T: 'static, S: 'static> Transaction<T, S> {
    #[inline]
    pub(crate) fn take_raw(&mut self) -> Result<TransactionRaw, Error> {
        use self::proto::Transaction::TransactionBody_oneof_data::*;

        match self.kind.take() {
            TransactionKind::Builder(_) => self.build().take_raw(),

            TransactionKind::Raw(mut state) => {
                let tx = &mut state.tx;

                // note: cannot fail
                let id = tx
                    .body
                    .as_ref()
                    .unwrap()
                    .transactionID
                    .as_ref()
                    .unwrap()
                    .clone();

                let operator = id.accountID.as_ref().unwrap().clone();

                if !tx.has_sigs() {
                    // If .sign was never called this will be still need to be initialized
                    tx.set_sigs(proto::BasicTypes::SignatureList::new());
                }

                if let Some(secret) = &self.secret {
                    let signature = secret()?.sign(&state.bytes).to_proto()?;

                    match &tx.body.as_ref().unwrap().data {
                        Some(cryptoTransfer(data)) => {
                            // Insert a signature for the operator if the operator
                            // is sending any monies
                            for transfer in &data.transfers.as_ref().unwrap().accountAmounts {
                                if transfer.accountID.as_ref().unwrap() == &operator {
                                    tx.sigs.as_mut().unwrap().sigs.push(signature.clone());
                                }
                            }
                        }

                        _ => {}
                    }

                    // Sign as the operator of the transaction
                    tx.sigs.as_mut().unwrap().sigs.insert(0, signature);
                }

                match tx.mut_body().data {
                    Some(cryptoDelete(ref mut data)) => {
                        if !data.has_transferAccountID() {
                            // default the transfer account ID to the operator of the transaction
                            data.set_transferAccountID(operator);
                        }
                    }

                    _ => {}
                }

                Ok(state)
            }

            TransactionKind::Err(err) => Err(err),

            TransactionKind::Empty => panic!("transaction already executed"),
        }
    }

    // Transition from builder to raw
    // Done before the first signature or execute
    #[inline]
    pub(crate) fn build(&mut self) -> &mut Transaction<T, TransactionRaw> {
        match &self.kind {
            TransactionKind::Empty => panic!("transaction already executed"),

            TransactionKind::Raw(_) | TransactionKind::Err(_) => {
                // Do nothing; we are already built
                // this is 100% safe; its changing a marker type parameter
                return unsafe { std::mem::transmute(self) };
            }

            _ => {
                // Fall-through to do something fun
            }
        }

        if let TransactionKind::Builder(state) = self.kind.take() {
            match state.to_proto() {
                Ok(tx) => {
                    // note: this cannot fail
                    let tx: proto::Transaction::Transaction = tx;
                    let bytes = tx.body.as_ref().unwrap().write_to_bytes().unwrap();

                    self.kind = TransactionKind::Raw(TransactionRaw { tx, bytes })
                }

                Err(error) => {
                    self.kind = TransactionKind::Err(error);
                }
            }
        }

        // this is 100% safe; its changing a marker type parameter
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> ToProto<proto::Transaction::Transaction> for TransactionBuilder<T> {
    fn to_proto(&self) -> Result<proto::Transaction::Transaction, Error> {
        let mut tx = proto::Transaction::Transaction::new();
        tx.set_body(self.to_proto()?);

        Ok(tx)
    }
}

impl<T> ToProto<proto::Transaction::TransactionBody> for TransactionBuilder<T> {
    fn to_proto(&self) -> Result<proto::Transaction::TransactionBody, Error> {
        // Get a reference to the trait implementation for ToProto for the inner builder
        let inner: &dyn ToProto<proto::Transaction::TransactionBody_oneof_data> =
            match self.inner.query_ref() {
                Some(inner) => inner,

                // Not possible in safe rust to get here
                _ => unreachable!(),
            };

        let tx_id = self
            .id
            .as_ref()
            .ok_or_else(|| ErrorKind::MissingField("operator"))?;

        let mut body = proto::Transaction::TransactionBody::new();
        let node = self.node.ok_or_else(|| ErrorKind::MissingField("node"))?;

        body.set_nodeAccountID(node.to_proto()?);
        body.set_transactionValidDuration(Duration::from_secs(120).to_proto()?);
        body.set_transactionFee(self.fee);
        body.set_generateRecord(self.generate_record);
        body.set_transactionID(tx_id.to_proto()?);
        body.data = Some(inner.to_proto()?);
        body.set_memo(if let Some(memo) = &self.memo {
            memo.to_owned()
        } else {
            String::new()
        });

        Ok(body)
    }
}
