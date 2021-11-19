ukkonen
=======

Implementation of a bounded Levenshtein distance by Esko Ukkonen in "Algorithms for approximate string matching" in Rust, with Python bindings.

The implementation is based on the [JS implementation](https://github.com/sunesimonsen/ukkonen). You can find the license in the `LICENSE` file.

## Install

```bash
pip install ukkonen-rs
```

## Build

First you should install Rust from rustup.rs.
Then, you can build a wheel for your platform and install it:
```bash
pip install maturin
maturin build --release --cargo-extra-args="--features python"
pip install --no-index --find-links=target/wheels ukkonen_rs
```
