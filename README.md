# bkm

📕 A terminal-based book manager, written in Rust 🦀

## Installing

If you're a **Rust programmer**, bkm can be installed with `cargo`.

- Note that the minimum supported version of Rust for bkm is **1.34.0**,
  although bkm may work with older versions.

```
$ cargo install ripgrep
```

### Building

bkm is written in Rust, so you'll need to grab a
[Rust installation](https://www.rust-lang.org/) in order to compile it.
bkm compiles with Rust 1.65.0 (stable) or newer. In general, bkm tracks
the latest stable release of the Rust compiler.

To build bkm:

```
$ git clone https://github.com/joao-vitor-sr/bkm
$ cd bkm
$ cargo build --release
$ ./target/release/bkm --version
0.1.1
```

## Usage

Call `bkm` in your terminal

```sh
$ bkm
```
