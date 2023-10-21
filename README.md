# Onano

#### A very simple text editor similar to `nano`

![Demo of usage](./docs/demo.gif)

## Usage
```bash
# Run without opening a file
onano

# Open a file
onano my-file.md
```

## Build
```bash
git clone https://github.com/jfvillablanca/onano
cd onano
cargo build --release
./target/release/onano
```

## Build with Nix
```bash
nix build github:jfvillablanca/onano
```

## Run with Nix
```bash
nix run github:jfvillablanca/onano my-file.md
```

This project is based on this wonderful [tutorial](https://www.flenker.blog/hecto/) which taught me the inner workings of text editors.
