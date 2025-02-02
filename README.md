# Prequsites
This was made to proof out if we could make a simple tooling for converting images to diecut files for print.

## 1: Install Rust
See: https://rustup.rs/

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

# Steps to run
## 1: Build
```bash
cargo build
```

## 2: Run
```bash
cargo run -- -i waifu.png -c 10
```

## 3: Convert
```bash
cargo run -- -i padded_waifu.png --png2svg
```


###
Rust code is written with Copilot and Deepseek R1 by [NeoTech](https://github.com/neotech/)