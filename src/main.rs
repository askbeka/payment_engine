use clap::{ Clap };
use csv;
mod dto;
use dto::{ input, output };
use std::{collections::HashMap, io};

/// Transactions application
#[derive(clap::Clap, Debug)]
#[clap(version = "1.0.0", author = "Beknar A. <beknaraskarov@gmail.com>")]
pub struct Config {
    /// Transactions file
    file_path: String,
}

fn main() {
    run(Config::parse()).unwrap();
}

fn run(config: Config) -> Result<(), csv::Error> {
    let mut reader = csv::Reader::from_path(config.file_path)?;
    let mut writer = csv::Writer::from_writer(std::io::stdout());

    

    Ok(())
}

fn read<R: io::Read>(reader: &csv::Reader<R>) -> Result<std::iter::Iterator<Item=&dto::output::AccountStatus>, csv::Error> {
    let AccountStatuses = reader.deserialize::<input::Transaction>()
        .map(|result| result.unwrap())
    
        .fold(HashMap::new(), |mut account_repository, transaction| {
            let account_status = account_repository.entry(transaction.client).or_insert(output::AccountStatus::new(transaction.client));

            account_status.process_transaction(transaction).unwrap();
            
            account_repository
        })
        .iter()
        .map(|(_, account_status)| account_status)
        .collect();
    
    Ok(AccountStatuses)
}

fn write() {

}