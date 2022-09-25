use super::TestValue;
use store::{store::Store, store_db::StoreDBBuilder};

#[test]
fn cycle() {
    let mut store = StoreDBBuilder::new(5).build().expect("Built");

    for i in 1..=10 {
        let old_value = store.insert(i, TestValue::new(i)).expect("Inserted");
        assert!(old_value.is_none(), "No previous value at {}", i);
    }

    let mut keys = store.keys().expect("Keys read");
    keys.sort();
    assert_eq!((1..=10).collect::<Vec<usize>>(), keys);

    for i in 1..=10 {
        let value = store.get(&i).expect("Gotten");
        assert!(value.is_some(), "Value exists at {}", i);
        assert_eq!(value.unwrap().id, i, "Correct value at {}", i);
    }
    for i in 1..=10 {
        let old_value = store.insert(i, TestValue::new(i + 1)).expect("Inserted");
        assert!(old_value.is_some(), "Value exists at {}", i);
        assert_eq!(old_value.unwrap().id, i, "Correct value at {}", i);
    }
    for i in 1..=10 {
        let value = store.remove(&i).expect("Removed");
        assert!(value.is_some(), "Value existed at {}", i);
        assert_eq!(value.unwrap().id, i + 1, "Correct value at {}", i);
    }
}
