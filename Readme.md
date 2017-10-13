intrusive-collections
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

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
