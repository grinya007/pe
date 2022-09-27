use std::error::Error as StdError;

use crate::{
    client::{Client, ClientCSV},
    input::{Action, Record},
    transaction::{Action as TransactionAction, Transaction},
};
use store::store::Store;

#[derive(Debug, Error)]
pub enum Error {
    /// A transaction of type Deposit or Withdrawal doesn't specify the amount of funds
    AmountUnspecified,
    /// A transaction of type Dispute, Resolve or Chargeback *does* specify the amount of funds
    AmountUnnecessary,
    /// The amount of funds specified is negative
    AmountNegative,
    /// A transaction of type Deposit or Withdrawal with the ID that has been seen previously
    TransactionIdDuplicate,
    /// A transaction of type Dispute, Resolve or Chargeback with the ID that hasn't been seen
    /// previously
    TransactionNotFound,
    /// A transaction of type Dispute, Resolve or Chargeback with the client ID that is different
    /// with that of the original Deposit transaction
    ClientIdMismatch,
    /// A transaction of type Dispute, Resolve or Chargeback with the client ID that can't be found
    // FIXME: perhaps this is the error of a higher order
    ClientNotFound,
}

pub struct Processor<CS: Store<u16, Client>, TS: Store<u32, Transaction>> {
    client_store: CS,
    transaction_store: TS,
}

impl<CS: Store<u16, Client>, TS: Store<u32, Transaction>> Processor<CS, TS> {
    pub fn new(client_store: CS, transaction_store: TS) -> Self {
        Self {
            client_store,
            transaction_store,
        }
    }

    /// # Errors
    pub fn process(&mut self, record: &Record) -> Result<(), Box<dyn StdError>> {
        if let Action::Deposit | Action::Withdrawal = record.action {
            self.process_init(record)?;
        } else if let Action::Dispute | Action::Resolve | Action::ChargeBack = record.action {
            self.process_mut(record)?;
        }

        Ok(())
    }

    /// # Errors
    pub fn clients_csv(
        &mut self,
    ) -> Result<Box<dyn Iterator<Item = ClientCSV> + '_>, Box<dyn StdError>> {
        Ok(Box::new(self.client_store.keys()?.into_iter().map(|id| {
            self.client_store
                .get(&id)
                .expect("Store should be accessible")
                .expect("Client should exist")
                .into()
        })))
    }

    fn process_init(&mut self, record: &Record) -> Result<(), Box<dyn StdError>> {
        if let Some(amount) = record.amount {
            if amount >= 0.0 {
                if self
                    .transaction_store
                    .get(&record.transaction_id)?
                    .is_none()
                {
                    let mut client =
                        if let Some(client) = self.client_store.get(&record.client_id)? {
                            client.clone()
                        } else {
                            Client::new(record.client_id)
                        };

                    let transaction_action = if let Action::Withdrawal = record.action {
                        client.withdraw(amount)?;
                        TransactionAction::Withdrawal
                    } else if let Action::Deposit = record.action {
                        client.deposit(amount)?;
                        TransactionAction::Deposit
                    } else {
                        unreachable!();
                    };

                    self.transaction_store.insert(
                        record.transaction_id,
                        Transaction::new(
                            record.transaction_id,
                            client.id(),
                            amount,
                            transaction_action,
                        ),
                    )?;
                    self.client_store.insert(client.id(), client)?;

                    Ok(())
                } else {
                    Err(Box::new(Error::TransactionIdDuplicate))
                }
            } else {
                Err(Box::new(Error::AmountNegative))
            }
        } else {
            Err(Box::new(Error::AmountUnspecified))
        }
    }

    fn process_mut(&mut self, record: &Record) -> Result<(), Box<dyn StdError>> {
        if record.amount.is_none() {
            if let Some(transaction) = self.transaction_store.get(&record.transaction_id)? {
                if transaction.client_id() == record.client_id {
                    if let Some(client) = self.client_store.get(&record.client_id)? {
                        let mut client = client.clone();
                        let mut transaction = transaction.clone();

                        if let Action::Dispute = record.action {
                            transaction.dispute()?;
                            client.dispute(transaction.amount())?;
                        } else if let Action::Resolve = record.action {
                            transaction.resolve()?;
                            client.resolve(transaction.amount())?;
                        } else if let Action::ChargeBack = record.action {
                            transaction.chargeback()?;
                            client.chargeback(transaction.amount())?;
                        }

                        self.transaction_store
                            .insert(transaction.id(), transaction)?;
                        self.client_store.insert(client.id(), client)?;

                        Ok(())
                    } else {
                        Err(Box::new(Error::ClientNotFound))
                    }
                } else {
                    Err(Box::new(Error::ClientIdMismatch))
                }
            } else {
                Err(Box::new(Error::TransactionNotFound))
            }
        } else {
            Err(Box::new(Error::AmountUnnecessary))
        }
    }
}
