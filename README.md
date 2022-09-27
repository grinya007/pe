# Simple Payment Engine

The project consists of three crates:
* `store` The key-value storage that's is used to memorize clients' accounts and transactions
* `engine` The core processor of the input transactions that can be used as a callable library
  within a server application
* `cli` A simple command-line interface that puts the above two in motion. The CLI can be used for
  testing/debugging or as a usage example of the `engine`

The correctness of the processing is tested with an [integration test](engine/tests/integration/processor.rs#L18) where a [sample input](engine/resources/processor/transactions_medium.csv) is used to provoke all sorts of exceptional cases that the processor handles.

Please run
```
cargo run -- -v engine/resources/processor/transactions_medium.csv
```
with `-v` to see the list of exceptions printed as warning to the STDERR

Please run
```
cargo doc --open
```
to see more details about the implementation of the Simple Payment Engine
