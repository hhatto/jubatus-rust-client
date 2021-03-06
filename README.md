# jubatus-rust-client [![Build Status](https://travis-ci.org/hhatto/jubatus-rust-client.svg?branch=master)](https://travis-ci.org/hhatto/jubatus-rust-client)

RPC client library for [Jubatus](http://jubat.us/) in [Rust](https://www.rust-lang.org/). 

## Usage

Cargo.toml:

```toml
[dependencies.jubatus]
git = "https://github.com/hhatto/jubatus-rust-client.git"
branch = "master"
```

```rust
extern crate jubatus;
```

## Example

```rust
extern crate jubatus;

use jubatus::classifier::client::ClassifierClient;

fn main() {
    let host = "127.0.0.1:9199";
    let name = "test";

    let mut client = ClassifierClient::new(host, name);
    println!("config:\n{}", client.get_config());
}
```

## for Maintainer

```console
$ sh -x generate.sh
$ cargo test
```

## License

MIT
