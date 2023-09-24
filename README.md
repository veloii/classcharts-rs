<div align="center">
  <h2 align="center">Unoffical ClassCharts Library for Rust</h2>
  
  <p align="center">
    An unoffical Student ClassCharts API library, built with Rust.
    <br />
    <a href="https://cc.veloi.me"><strong>Explore the docs »</strong></a>
    <br />
    <br />
    <a href="https://github.com/veloii/classcharts-rs/issues">Report Bug</a>
    ·
    <a href="https://crates.io/crates/classcharts">crates.io</a>
    ·
    <a href="https://github.com/veloii/classcharts-rs/issues">Request Feature</a>
  </p>
</div>


## What is this?

An API wrapper with *mostly* proper typings and tests for ClassCharts.
Looking for a more complete guide? [Look at the docs.](https://cc.veloi.me)

## Usage
```bash
cargo add classcharts
```
or in your `Cargo.toml`
```toml
[dependencies]
...
classcharts = "1.0.2"
```

Use the [examples/basic.rs](https://github.com/veloii/classcharts-rs/blob/main/examples/basic.rs) as a reference.

```bash
git clone git@github.com:veloii/classcharts-rs.git
cd classcharts-rs && cargo run --example basic
```

## Developing

```bash
git clone git@github.com:veloii/classcharts-rs.git
cd classcharts-rs
```
```bash
cargo test
```

## Credits

- I used the type definitions from [classchartsapi/classcharts-api-js](https://github.com/classchartsapi/classcharts-api-js) to create `structs`/`enums`.
