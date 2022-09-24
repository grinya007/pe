use serde::{Deserialize, Serialize};

mod store_db;
mod store_mem;

#[derive(Deserialize, Serialize)]
struct TestValue {
    id: usize,
}

impl TestValue {
    fn new(id: usize) -> Self {
        Self { id }
    }
}
