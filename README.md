# prime-utils

Utilities for working with prime numbers in Rust.  
Includes a fast deterministic Miller–Rabin implementation for `u64` usable with the command line tool `largest-prime-below`.

## Installation
  
### From local source

```bash
cargo install --path .
```

### From GitHub

```bash
cargo install --git https://github.com/andyhedges/prime-utils
```

To install a specific binary:

```bash
cargo install --git https://github.com/andyhedges/prime-utils --bin largest-prime-below
```

## Usage

### largest-prime-below

```bash
largest-prime-below 100
# 97
```

Help:

```bash
largest-prime-below --help
```

Version:

```bash
largest-prime-below --version
```

## Library

```rust
use prime_utils::largest_prime_below;

let p = largest_prime_below(100);
println!("{}", p);
```

