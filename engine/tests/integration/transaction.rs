use file_diff::diff;
use random_string::generate;
use std::{env::temp_dir, fs::remove_file};

use engine::{
    transaction::{Action, Transaction},
    write_csv::{write_csv, Output},
};
use store::{store::Store, store_db::StoreDBBuilder};

#[test]
fn with_store_db() {
    let mut store = StoreDBBuilder::new(5).build().expect("Built");

    for i in 1..=10 {
        let old_value = store
            .insert(i, Transaction::new(i, 10, 0.1, Action::Deposit))
            .expect("Inserted");
        assert!(old_value.is_none(), "No previous value at {}", i);
    }

    let mut keys = store.keys().expect("Keys read");
    keys.sort();
    assert_eq!((1..=10).collect::<Vec<u32>>(), keys);
    assert_eq!(
        (1..=10).collect::<Vec<u32>>(),
        keys.iter()
            .map(|id| store.get(id).expect("Gotten").expect("Found").id())
            .collect::<Vec<_>>()
    );

    for i in 1..=10 {
        let value = store.get(&i).expect("Gotten");
        assert!(value.is_some(), "Value exists at {}", i);
        assert_eq!(value.unwrap().id(), i, "Correct value at {}", i);
    }
    for i in 1..=10 {
        let old_value = store
            .insert(i, Transaction::new(i + 1, 10, 0.2, Action::Withdrawal))
            .expect("Inserted");
        assert!(old_value.is_some(), "Value exists at {}", i);
        assert_eq!(old_value.unwrap().id(), i, "Correct value at {}", i);
    }
    for i in 1..=10 {
        let value = store.remove(&i).expect("Removed");
        assert!(value.is_some(), "Value existed at {}", i);
        assert_eq!(value.unwrap().id(), i + 1, "Correct value at {}", i);
    }
}

#[test]
fn store_db_to_csv() {
    let mut store = StoreDBBuilder::new(5).build().expect("Built");

    for i in 1..=10 {
        let mut tx = Transaction::new(i, 10, 0.1, Action::Deposit);
        if i % 2 == 0 {
            tx.dispute().expect("Dispute OK");
            if i % 3 == 0 {
                tx.resolve().expect("Resolve OK");
            } else if i % 4 == 0 {
                tx.chargeback().expect("Chargeback OK");
            }
        }
        store.insert(i, tx).expect("Inserted");
    }

    let mut keys = store.keys().expect("Keys read");
    keys.sort();

    let tmp_file = format!(
        "{}/transactions_{}.csv",
        temp_dir().display(),
        generate(16, "abcdefghijklmnopqrstuvwxyz1234567890")
    );
    write_csv(
        &Output::File(&tmp_file),
        keys.iter()
            .map(|id| store.get(id).expect("Gotten").expect("Found").clone()),
    )
    .expect("Written");

    let etalon_file = format!("{}/resources/transactions.csv", env!("CARGO_MANIFEST_DIR"));
    assert!(diff(&etalon_file, &tmp_file));

    remove_file(tmp_file).expect("Temporary file removed");
}
