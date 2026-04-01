# rust sound

## Setup

### Install

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt
cargo install just

sudo apt install -y libasound2-dev pkg-config
```

### Run on debug

```
just run --file examples/twinkle.txt  --pitch 1.2 --tempo 1.2 --volume 0.05 --soft 2 --square 0.05
```

### Build

```
just build
```

### Run

```
target/release/sound --file examples/twinkle.txt  --pitch 1.2 --tempo 1.2 --volume 0.05 --soft 2 --square 0.05
```

### Description

```
.
├── Cargo.toml
├── README.md
├── examples
│   ├── aimer-fast.txt
│   ├── aimer.txt
│   └── twinkle.txt
├── justfile
└── src
    └── main.rs
```
