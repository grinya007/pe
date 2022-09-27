use clap::Parser;

use engine::{
    processor::Processor,
    write_csv::{write_csv, Output},
};
use store::{store_db::StoreDBBuilder, store_mem::StoreMem};

#[derive(Parser)]
#[clap(name = "Payment Engine")]
#[clap(author = "Gregory Arefyev <gregory@recom.live>")]
#[clap(version = "0.1.0")]
pub struct Args {
    #[clap(value_parser, help = "Input file")]
    pub input_file: String,
    #[clap(short, long, help = "Increase log level")]
    pub verbose: bool,
}

fn main() {
    let args = Args::parse();
    let log_level = if args.verbose {
        tracing::Level::WARN
    } else {
        tracing::Level::ERROR
    };

    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(log_level)
        .init();

    let mut processor = Processor::new(
        StoreMem::new(),
        StoreDBBuilder::new(1_000_000)
            .build()
            .expect("StoreDB created"),
    );

    let mut reader = csv::Reader::from_path(&args.input_file).expect("CSV reader created");

    for record in reader.deserialize() {
        if let Err(error) = record {
            log::error!("Failed to parse CSV [{}]: {}", args.input_file, error);
        } else if let Err(error) = processor.process(&record.unwrap()) {
            log::warn!("Failed to process record: {}", error);
        }
    }

    write_csv(
        &Output::STDOUT,
        processor.clients_csv().expect("Clients read"),
    )
    .expect("Written");
}
