use serde::Serialize;
use std::collections::{HashMap, HashSet};
use super::input;

#[derive(Debug, Serialize)]
pub struct AccountStatus {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
    #[serde(skip_serializing)]
    transactions_amounts: HashMap<u32, f32>,
    #[serde(skip_serializing)]
    disputed_transactions: HashSet<u32>,
}

impl AccountStatus {
    pub fn new(client: u16) -> AccountStatus {
        AccountStatus {
            client,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
            transactions_amounts: HashMap::new(),
            disputed_transactions: HashSet::new(),
        }
    }
    pub fn process_transaction(&mut self, transaction: input::Transaction) -> Result<(), String> {
        if transaction.client != self.client {
            return Err(format!("transaction client does not match, expected: {} got: {}", self.client, transaction.client));
        }

        match transaction.transaction_type {
            input::TransactionType::Deposit => {
                if let Some(amount) = transaction.amount {
                    self.available += amount;
                    self.total += amount;
                }
            }
            input::TransactionType::Withdrawal => {
                if let Some(amount) = transaction.amount {
                    if self.available >= amount {
                        self.available -= amount;
                        self.total -= amount;
                    }
                }
            }
            input::TransactionType::Dispute => {
                if let Some(amount) = self.transactions_amounts.get(&transaction.tx) {
                    self.available -= amount;
                    self.held += amount;
                    self.disputed_transactions.insert(transaction.tx);
                }
            }
            input::TransactionType::Resolve => {
                if let Some(amount) = self.transactions_amounts.get(&transaction.tx) {
                    if self.disputed_transactions.contains(&transaction.tx) {
                        self.available += amount;
                        self.held -= amount;
                        self.disputed_transactions.remove(&transaction.tx);
                    }
                }
            }
            input::TransactionType::Chargeback => {
                if let Some(amount) = self.transactions_amounts.get(&transaction.tx) {
                    if self.disputed_transactions.contains(&transaction.tx) {
                        self.available -= amount;
                        self.held -= amount;
                        self.locked = true;
                        self.disputed_transactions.remove(&transaction.tx);
                    }
                }
            }
        }

        Ok(())
    }
}