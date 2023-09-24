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

## Installation
```bash
cargo add classcharts
```
or in your `Cargo.toml`
```toml
[dependencies]
...
classcharts = "latest"
```

## Usage
There is also a [examples/basic.rs](https://github.com/veloii/classcharts-rs/blob/main/examples/basic.rs) as a reference.

To create a ClassCharts Student Client and get their info.

```rust
use classcharts::Client;

let mut client = Client::create("your access code", "your date of birth (DD/MM/YYYY)", None).await.unwrap();

let student_info = client.get_student_info().await.unwrap();
println!("{:?}", student_info);
```

To view the current student's homework:

```rust
let homework = client.get_homeworks(None).await.unwrap();
```

For a complete list of ClassCharts methods the `Client` exposes:
 * `get_activity`
 * `get_full_activity`
 * `get_announcements`
 * `get_attendance`
 * `get_badges`
 * `get_behaviour`
 * `get_detentions`
 * `get_homeworks`
 * `get_lessons`
 * `get_pupilfields`
 * `get_rewards`
 * `purchase_reward`
 * `get_student_info`

They will all return a `Result<SuccessResponse, ErrorResponse>`.

For more information, please look at [the docs](https://cc.veloi.me).

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
