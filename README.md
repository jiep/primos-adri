<div align="center">

  <h1><code>primos-adri</code></h1>

  <strong>Implementation of `primos-adri`</strong>

  [![ci](https://github.com/jiep/primos-adri/actions/workflows/ci.yml/badge.svg)](https://github.com/jiep/primos-adri/actions/workflows/ci.yml)
  [![dependency status](https://deps.rs/repo/github/jiep/primos-adri/status.svg)](https://deps.rs/repo/github/jiep/primos-adri)

  <sub>Built with 🦀</sub>
</div>

## Binaries

Download the latest version from [Releases](https://github.com/jiep/primos-adri/releases).

## Build from source

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Check source code

```
cargo check
``` 

3. Compile binary

```
cargo build
``` 

4. Run tests

```
cargo test
```

> Note: for release target, add --release

5. Run binary

```
cargo run
# or
./target/release/primos-adri # for release version
./target/debug/primos-adri # for debug version
```

## 🚴 Usage

```
./target/release/primos-adri --help
A generator of Primos de Adri

Usage: primos-adri [OPTIONS]

Options:
  -m, --maximum <MAXIMUM>  [default: 100000]
  -h, --help               Print help information
  -V, --version            Print version information
```

### Example

```
66 Primos de Adri hasta 100000:
[22277, 22727, 22777, 23227, 23327, 23357, 23537, 23557, 25237, 25357, 25537, 25577, 27277, 27337, 27527, 27737, 32237, 32257, 32327, 32377, 32537, 33377, 33577, 33757, 35227, 35257, 35327, 35527, 35537, 37277, 37337, 37357, 37537, 52237, 52727, 52757, 53327, 53377, 53527, 53777, 55337, 57527, 57557, 57727, 57737, 72227, 72277, 72337, 72577, 72727, 73237, 73277, 73327, 73727, 73757, 75227, 75277, 75337, 75377, 75527, 75557, 75577, 77237, 77377, 77527, 77557]
```
