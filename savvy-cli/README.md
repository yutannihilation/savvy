# savvy-cli

A helper CLI for savvy framework. For the full details, please read [savvy's crate
documentation](https://docs.rs/savvy/latest/).

## Installation

You can find the binary on [the GitHub releases
page](https://github.com/yutannihilation/savvy/releases). If you prefer installing from source, please run cargo install.

``` shell
cargo install savvy-cli
```

## Usage

``` console
Generate C bindings and R bindings for a Rust library

Usage: savvy-cli <COMMAND>

Commands:
  c-header      Generate C header file
  c-impl        Generate C implementation for init.c
  r-impl        Generate R wrapper functions
  makevars      Generate Makevars
  makevars-win  Generate Makevars.win
  gitignore     Generate .gitignore
  update        Update wrappers in an R package
  init          Init savvy-powered Rust crate in an R package
  help          Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
