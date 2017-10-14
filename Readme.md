rbtree for rust
=====================

[![Build Status](https://travis-ci.org/tickbh/rbtree-rs.svg?branch=master)](https://travis-ci.org/tickbh/rbtree-rs) [![Crates.io](https://img.shields.io/crates/v/rbtree.svg)](https://crates.io/crates/rbtree)

A Rust library for creating red-black trees. 

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rbtree = "0.1"
```

and this to your crate root:

```rust
extern crate rbtree;
```

How to use
```rust
use rbtree::RBTree;
let mut m = RBTree::new();
assert_eq!(m.len(), 0);
m.insert(1, 2);
assert_eq!(m.len(), 1);
m.insert(2, 4);
assert_eq!(m.len(), 2);
assert_eq!(*m.get(&1).unwrap(), 2);
assert_eq!(*m.get(&2).unwrap(), 4);
```

## Bench
bench code in ./examples/bench.rs, you can run command
```txt
cargo run --release --example bench
```
```txt
All Test Repeat: 10, All Insert Num: 1000, Max Cost: 304us, Min Cost: 106us, Aver Cost: 147us
Get data by key=20 From Tree Num: 1000, Max Cost: 395ns, Min Cost: 0ns, Aver Cost: 158ns
All Test Repeat: 10, All Insert Num: 10000, Max Cost: 1738us, Min Cost: 1463us, Aver Cost: 1621us
Get data by key=20 From Tree Num: 10000, Max Cost: 1185ns, Min Cost: 395ns, Aver Cost: 592ns
All Test Repeat: 10, All Insert Num: 100000, Max Cost: 25048us, Min Cost: 19115us, Aver Cost: 21351us
Get data by key=20 From Tree Num: 100000, Max Cost: 1975ns, Min Cost: 1185ns, Aver Cost: 1461ns
All Test Repeat: 10, All Insert Num: 1000000, Max Cost: 283113us, Min Cost: 250630us, Aver Cost: 262112us
Get data by key=20 From Tree Num: 1000000, Max Cost: 2765ns, Min Cost: 1975ns, Aver Cost: 2054ns
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
