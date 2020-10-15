use serde::Serialize;
use std::collections::{HashMap, HashSet};
use super::input;
use rust_decimal::Decimal;
use crate::utils::serializer;

#[derive(Debug, Serialize)]
pub struct AccountStatus {
    client: u16,
    #[serde(serialize_with = "serializer::fixed_precision_serializer")]
    available: Decimal,
    #[serde(serialize_with = "serializer::fixed_precision_serializer")]
    held: Decimal,
    #[serde(serialize_with = "serializer::fixed_precision_serializer")]
    total: Decimal,
    locked: bool,
    #[serde(skip_serializing)]
    transactions_amounts: HashMap<u32, Decimal>,
    #[serde(skip_serializing)]
    disputed_transactions: HashSet<u32>,
}

impl AccountStatus {
    pub fn new(client: u16) -> AccountStatus {
        AccountStatus {
            client,
            available: Decimal::new(0, 0),
            held: Decimal::new(0, 0),
            total: Decimal::new(0, 0),
            locked: false,
            transactions_amounts: HashMap::new(),
            disputed_transactions: HashSet::new(),
        }
    }

    pub fn process_transaction(&mut self, transaction: &input::Transaction) {
        match transaction.transaction_type {
            input::TransactionType::Deposit => {
                if let Some(amount) = transaction.amount {
                    self.transactions_amounts.insert(transaction.tx, amount);
                    self.available += amount;
                    self.total += amount;
                } else {
                    eprintln!("deposit: {} is not possible, invalid amount", transaction.tx);
                }
            }
            input::TransactionType::Withdrawal => {
                if let Some(amount) = transaction.amount {
                    if self.available >= amount {
                        self.transactions_amounts.insert(transaction.tx, amount);
                        self.available -= amount;
                        self.total -= amount;
                    } else {
                        eprintln!("withdrawal: {} is not possible, not enough available", transaction.tx);
                    }
                } else {
                    eprintln!("withdrawal: {} is not possible, invalid amount", transaction.tx);
                }
            }
            input::TransactionType::Dispute => {
                if let Some(amount) = self.transactions_amounts.get(&transaction.tx) {
                    self.available -= amount;
                    self.held += amount;
                    self.disputed_transactions.insert(transaction.tx);
                } else {
                    eprintln!("dispute: {} is not possible transaction is not present", transaction.tx);
                }
            }
            input::TransactionType::Resolve => {
                if let Some(amount) = self.transactions_amounts.get(&transaction.tx) {
                    if self.disputed_transactions.contains(&transaction.tx) {
                        self.available += amount;
                        self.held -= amount;
                        self.disputed_transactions.remove(&transaction.tx);
                    } else {
                        eprintln!("reolve: {} is not possible, no dispute found", transaction.tx);
                    }
                } else {
                    eprintln!("resolve: {} is not possible transaction is not present", transaction.tx);
                }
            }
            input::TransactionType::Chargeback => {
                if let Some(amount) = self.transactions_amounts.get(&transaction.tx) {
                    if self.disputed_transactions.contains(&transaction.tx) {
                        self.total -= amount;
                        self.held -= amount;
                        self.locked = true;
                        self.disputed_transactions.remove(&transaction.tx);
                    } else {
                        eprintln!("chargeback: {} is not possible, no dispute found", transaction.tx);
                    }
                } else {
                    eprintln!("chargeback: {} is not possible transaction is not present", transaction.tx);
                }
            }
        };
    }
}