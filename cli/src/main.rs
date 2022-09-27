//! CLI interface to the [Simple Payment Engine](../engine/index.html)
//! built on top of the [Store Engine](../store/index.html)

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
        // the size of the in-memory part of the StoreDB could be a cli argument
        // as well as the choice of the store engines for clients and transactions
        // the below hadcoded configuration is inspired by the description of the problem at hand
        StoreMem::new(),
        StoreDBBuilder::new(1_000_000)
            .build()
            .expect("StoreDB created"),
    );

    let mut reader = csv::Reader::from_path(&args.input_file).expect("CSV reader created");

    // possible improvement:
    //  the processing can be parallelized in N threads
    //  each thread will have its own instance of the `Processor`
    //  there's no data that the threads would need to share
    //  as long as the sharding is done based on `record.client_id`
    //  i.e. `process_thread_id = record.client_id % n_threads`
    for record in reader.deserialize() {
        if let Err(error) = record {
            log::error!("Failed to parse CSV [{}]: {}", args.input_file, error);
        } else if let Err(error) = processor.process(record.as_ref().unwrap()) {
            // possible improvement:
            //  errors that come from the Store engine should have a higher rank
            //  right now they are mixed together with the errors of the Processor
            log::warn!("Failed to process record [{:?}]: {}", record, error);
        }
    }

    write_csv(
        // the output (stdout or a file) could be an optional cli argument
        &Output::STDOUT,
        processor.clients_csv().expect("Clients read"),
    )
    .expect("Written");
}
