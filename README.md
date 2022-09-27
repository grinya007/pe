# Simple Payment Engine

The project consists of three crates:
* `store`: The key-value storage that's is used to memorize clients' accounts and transactions
* `engine`: The core processor of the input transactions that can be used as a callable library
  within a server application
* `cli`: A simple command-line interface that puts the above two in motion. The CLI can be used for
  testing/debugging or as a usage example of the `engine`

