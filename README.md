<div align="center">

  <h1><code>primos-adri</code></h1>

  <strong>A generator of Primos de Adri</strong>

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

3. Build binary

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
  cargo run                     # for debug version

  # or

  ./target/release/primos-adri  # for release version
  ```

## 🚴 Usage

```
primos-adri --help
A generator of Primos de Adri

Usage: primos-adri <COMMAND>

Commands:
  check     Check if a given number is Primo de Adri
  generate  Generate Primos de Adri with given digits
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

### Generate

```
primos-adri generate --help
Generate Primos de Adri with given digits

Usage: primos-adri generate [OPTIONS]

Options:
  -d, --digits <DIGITS>  [default: 5]
  -h, --help             Print help information
  -V, --version          Print version information
```

### Check

```
Check if a given number is Primo de Adri

Usage: primos-adri check --number <NUMBER>

Options:
  -n, --number <NUMBER>  
  -h, --help             Print help information
  -V, --version          Print version information
```

### Examples

#### Check if 772757 is Primo de Adri

```
primos-adri check --number 772757
772757 is a Primo de Adri!
```

#### Check if 332757 is Primo de Adri

```
primos-adri check --number 332757
332757 is NOT a Primo de Adri!
```

#### Generate Primos de Adri with 5 digits

```
primos-adri generate --digits 5
66 Primos de Adri up to 100000:
22277
22727
22777
23227
23327
23357
23537
23557
25237
25357
25537
25577
27277
27337
27527
27737
32237
32257
32327
32377
32537
33377
33577
33757
35227
35257
35327
35527
35537
37277
37337
37357
37537
52237
52727
52757
53327
53377
53527
53777
55337
57527
57557
57727
57737
72227
72277
72337
72577
72727
73237
73277
73327
73727
73757
75227
75277
75337
75377
75527
75557
75577
77237
77377
77527
77557
```

## License
This project is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.