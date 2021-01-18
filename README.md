# clarity-linter

_This cli is a work in progress and should not be considered a finished product_

A command line tool to lint clarity smart contracts.

## Installation

Binaries for various platforms can be downloaded from releases section. clarity-linter is available for linux, windows and mac.

[Releases](https://github.com/alexkeating/clarity-linter/releases)

If Rust is setup clarity-lint can be installed via cargo

```
cargo install clarity-lint
```

## Quickstart

Basic functionality of the cli tool is given below:

```sh
➜ clarity-lint --help

USAGE:
    clarity-lint --file <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <PATH>    Species the clarity file to be linted
```
