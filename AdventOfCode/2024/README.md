# Advent of Code 2024

## Getting Started

### Requirements

- Rust nightly
- just

### Setup

The create script requires the nightly toolchain.

```
rustup default nightly
cargo install just
cargo install cargo-binstall
cargo binstall cargo-nextest --secure
cargo install cargo-watch
cargo install --locked samply
```

### nix

```
nix develop
```

# Test all days

cargo nextest run

# Create

```
just create day-xx
```

# Run

```
just run day-xx
```

# Dev

```
just work day-xx
```

