# Substrate Kitties Tutorial

This README is based on
[dotcodeschool](https://dotcodeschool.com/courses/substrate-kitties) tutorial `substrate-kitties`.
Kudos to [Shawn Tabrizi](https://github.com/shawntabrizi)
for this amazing tutorial.

## Material

### Framework

[Polkadot-SDK](https://github.com/paritytech/polkadot-sdk)

### Videos

[Blockchain](https://youtu.be/8UvdfFGYFiE)
[Substrate](https://youtu.be/-ttmm8gYS04)
[FRAME](https://youtu.be/ghMloMzEEsA)

### Documentation

[Pallet](https://docs.rs/pallet-sudo/latest/pallet_sudo/index.html)
[Macros](https://doc.rust-lang.org/book/ch20-05-macros.html)
[Supertraits](https://doc.rust-lang.org/rust-by-example/trait/supertraits.html)

## Commands

```bash
rustup update

wc -l # shows the number of lines in a file
cargo expand # expand macros

cargo +nightly fmt
cargo +nightly clippy
cargo test
```

## After each step

Run `fmt` `clippy` and `test` commands.
You should not get any errors (warnings are ok).

## Starting template

Is on the `empty` branch.

## Teminology

Users submit an `extrinsic` to the blockchain,
which is `dispatched` to a Pallet `call`.

`extrinsic` is any message from the outside coming to the blockchain.
`transaction` is specifically a **signed** message coming from the outside.
