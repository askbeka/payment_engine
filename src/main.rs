use clap::{ Clap };
use pe_lib::run;

/// Transactions application
#[derive(clap::Clap, Debug)]
#[clap(version = "1.0.0", author = "Beknar A. <beknaraskarov@gmail.com>")]
pub struct Config {
    /// Transactions file
    file_path: String,
}

fn main() {
    let config: Config = Config::parse();
    let mut reader = csv::Reader::from_path(config.file_path).unwrap();
    let mut writer = csv::Writer::from_writer(std::io::stdout());
    run(&mut reader, &mut writer).unwrap();
}