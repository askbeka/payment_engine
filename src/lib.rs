use csv::{Reader, Writer};
use std::{collections::HashMap, io};

mod dto;
use dto::{input, output};

pub mod utils;

pub fn run<R: io::Read, W: io::Write>(reader: &mut Reader<R>, writer: &mut Writer<W>) -> Result<(), csv::Error> {
    reader.deserialize::<input::Transaction>()
        .map(|result| result.unwrap())
        .fold(HashMap::new(), |mut account_repository, transaction| {
            let account_status = account_repository.entry(transaction.client).or_insert(output::AccountStatus::new(transaction.client));

            account_status.process_transaction(&transaction);
            
            account_repository
        })
        .iter()
        .map(|(_, account_status)| account_status)
        .for_each(|account_status| {
            writer.serialize(account_status).unwrap();
        });

    Ok(())
}