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
bench code in ./examples/bench.rs, you can run command, in cpu i7-6700HQ, 2.60GHz
```txt
cargo run --release --example bench
```
```txt
-----------All Test Repeat: 10, All Tree Num: 1000-------------------
Insert Test,           Max Cost: 124us, Min Cost: 101us, Aver Cost: 106us
Get data by key=20,    Max Cost: 395ns, Min Cost: 0ns, Aver Cost: 39ns
Remove data by key=20, Max Cost: 790ns, Min Cost: 0ns, Aver Cost: 276ns
-----------End Tree Test----------------------------------------------

-----------All Test Repeat: 10, All Tree Num: 10000-------------------
Insert Test,           Max Cost: 2255us, Min Cost: 1383us, Aver Cost: 1590us
Get data by key=20,    Max Cost: 1975ns, Min Cost: 0ns, Aver Cost: 592ns
Remove data by key=20, Max Cost: 2765ns, Min Cost: 395ns, Aver Cost: 829ns
-----------End Tree Test----------------------------------------------

-----------All Test Repeat: 10, All Tree Num: 100000-------------------
Insert Test,           Max Cost: 21583us, Min Cost: 18904us, Aver Cost: 19859us
Get data by key=20,    Max Cost: 16987ns, Min Cost: 1185ns, Aver Cost: 2883ns
Remove data by key=20, Max Cost: 2370ns, Min Cost: 1185ns, Aver Cost: 1817ns
-----------End Tree Test----------------------------------------------

-----------All Test Repeat: 10, All Tree Num: 1000000-------------------
Insert Test,           Max Cost: 273758us, Min Cost: 249229us, Aver Cost: 257955us
Get data by key=20,    Max Cost: 4345ns, Min Cost: 1580ns, Aver Cost: 2409ns
Remove data by key=20, Max Cost: 7506ns, Min Cost: 1975ns, Aver Cost: 2923ns
-----------End Tree Test----------------------------------------------

-----------All Test Repeat: 10, All Tree Num: 10000000-------------------
Insert Test,           Max Cost: 3345163us, Min Cost: 3118450us, Aver Cost: 3221181us
Get data by key=20,    Max Cost: 11456ns, Min Cost: 2370ns, Aver Cost: 3831ns
Remove data by key=20, Max Cost: 9086ns, Min Cost: 2370ns, Aver Cost: 3594ns
-----------End Tree Test----------------------------------------------
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
