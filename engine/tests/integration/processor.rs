use file_diff::diff;
use itertools::Itertools;
use random_string::generate;
use std::{env::temp_dir, fs::remove_file};

use engine::{
    processor::Processor,
    write_csv::{write_csv, Output},
};
use store::{store_db::StoreDBBuilder, store_mem::StoreMem};

#[test]
fn small() {
    test_processor("small", 3);
}

#[allow(unused_must_use)]
fn test_processor(dataset: &str, txbuffer: usize) {
    let mut processor = Processor::new(
        StoreMem::new(),
        StoreDBBuilder::new(txbuffer)
            .build()
            .expect("StoreDB created"),
    );

    let mut reader = csv::Reader::from_path(format!(
        "{}/resources/processor/transactions_{}.csv",
        env!("CARGO_MANIFEST_DIR"),
        dataset
    ))
    .expect("CSV reader created");

    for record in reader.deserialize() {
        processor.process(&record.expect("Valid record"));
    }

    let tmp_file = format!(
        "{}/accounts_{}_{}.csv",
        temp_dir().display(),
        dataset,
        generate(16, "abcdefghijklmnopqrstuvwxyz1234567890")
    );

    write_csv(
        &Output::File(&tmp_file),
        processor
            .clients_csv()
            .expect("Clients read")
            .sorted_by(|a, b| Ord::cmp(&a.id, &b.id)),
    )
    .expect("Written");

    let etalon_file = format!(
        "{}/resources/processor/accounts_{}.csv",
        env!("CARGO_MANIFEST_DIR"),
        dataset
    );
    assert!(diff(&etalon_file, &tmp_file));

    remove_file(tmp_file).expect("Temporary file removed");
}
