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
cargo run -- -i padded_waifu.png -p
```

## 4: Build SVG
```bash
./potrace.exe -s --flat converted_padded_waifu.bmp -o output.svg
```

###
Potrace is a tool that can be downloaded from [here](https://potrace.sourceforge.net/) 
Copyright © 2001-2019 Peter Selinger.