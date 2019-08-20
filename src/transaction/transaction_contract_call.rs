use crate::{
    proto::{self, ToProto, Transaction::Transaction_oneof_bodyData,
            TransactionBody::TransactionBody_oneof_data},
    transaction::Transaction,
    Client, ContractId,
};
use failure::Error;
use query_interface::{interfaces, vtable_for};
use std::any::Any;

pub struct TransactionContractCall {
    id: ContractId,
    gas: i64,
    amount: i64,
    function_parameters: Vec<u8>,
}

interfaces!(
    TransactionContractCall: dyn Any,
    dyn ToProto<TransactionBody_oneof_data>
);

impl TransactionContractCall {
    pub fn new(client: &Client, id: ContractId) -> Transaction<Self> {
        Transaction::new(
            client,
            Self {
                gas: 0,
                amount: 0,
                function_parameters: Vec::new(),
                id,
            },
        )
    }
}

impl Transaction<TransactionContractCall> {
    /// The maximum amount of gas to use for the call.
    #[inline]
    pub fn gas(&mut self, gas: i64) -> &mut Self {
        self.inner().gas = gas;
        self
    }

    /// Number of tinybars to send (the function must be payable if this is nonzero).
    #[inline]
    pub fn amount(&mut self, amount: i64) -> &mut Self {
        self.inner().amount = amount;
        self
    }

    /// Which function to call, and the parameters to pass to the function.
    #[inline]
    pub fn function_parameters(&mut self, params: Vec<u8>) -> &mut Self {
        self.inner().function_parameters = params;
        self
    }
}

impl ToProto<TransactionBody_oneof_data> for TransactionContractCall {
    fn to_proto(&self) -> Result<TransactionBody_oneof_data, Error> {
        let mut data = proto::ContractCall::ContractCallTransactionBody::new();
        data.set_contractID(self.id.to_proto()?);
        data.set_gas(self.gas);
        data.set_amount(self.amount);
        data.set_functionParameters(self.function_parameters.clone());

        Ok(TransactionBody_oneof_data::contractCall(data))
    }
}
