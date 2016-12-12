jubatus-rust-client
===================

RPC client library for [Jubatus](http://jubat.us/) in [Rust](https://www.rust-lang.org/). 

**This is experimental module. Highly under development.**

## Usage

Cargo.toml:

```toml
[dependencies.jubatus]
git = "https://github.com/hhatto/jubatus-rust-client.git"
branch = "master"
```

```rust
extern crate jubatus;

use jubatus::client;
```

## Example

```rust
extern crate jubatus;

use jubatus::classifier::client::ClassifierClient;

fn main() {
    let host = "127.0.0.1:9199";
    let name = "test";

    let mut client = ClassifierClient::new(host, name);
    println!("{:?}", client.get_config());
}
```

## License

MIT
