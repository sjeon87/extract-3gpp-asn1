# extract-3gpp-asn1

Extract 3GPP ASN.1 from a plain text file and print to stdout

## Usage

```sh
Usage: extract-3gpp-asn1 [OPTIONS] [FILE]

Arguments:
  [FILE]  The file to process

Options:
  -c, --comment-process-level...  Comment process level
                                  - 0 (no option given): All comments are preserved
                                  - 1 (-c): Comments containing 'need' or 'cond' are preserved
                                  - 2 (-cc or more): Comments containing 'need' or 'cond' are removed (match whole word)
                                  - 3 or higher (-ccc or more): All comments are removed
                                  Note: This is not syntax-aware but simple pattern-matching
  -h, --help                      Print help
  -V, --version                   Print version

Note: Input can be provided via a file path or standard input (stdin) with pipe (|).
```

## Cross-compilation

### Windows on Linux/macOS

```sh
# Add target
rustup target add x86_64-pc-windows-gnu
# Install linker
apt install -y mingw-w64 # Linux
brew install mingw-w64 # macOS
```

`Cargo.toml`

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```

```sh
cargo build --release --target x86_64-pc-windows-gnu
```
