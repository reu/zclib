# zclib

Zlib command line interface.

## Installation

### Using prebuilt binaries

Check out the [releases](https://github.com/reu/zclib/releases) page for prebuilt versions of zclib for many different architectures.

### From source

To build from source, first make sure you have the Rust toolchan installed. Then just use Cargo:

```bash
cargo install --locked zclib
```

## Usage

```
zclib 0.1.0
Zlib command line interface

USAGE:
    zclib [FLAGS] [OPTIONS] [FILE]

FLAGS:
    -b, --best          Best (slowest) compression
    -d, --decompress    Decompress
    -f, --fast          Fastest (worst) compression
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
    -l, --level <level>    Compressing level (0-9)

ARGS:
    <FILE>    File
```

#### Compressing a file

```bash
zclib file > file.zlib
```

The input file can also be read from stdin:

```bash
zclib < file > file.zlib
echo "Hello World" | zclib > hello.zlib
```

#### Decompressing a file

```bash
echo "Hello World" | zclib > hello.zlib
zclib -d hello.zlib > hello
```

## Credits

All the credits are given to the contributors of the [flate2](https://github.com/rust-lang/flate2-rs), since I didn't implement any part of the zlib algorithim.

The rest of the credits goes to the [clap](https://github.com/clap-rs/clap) library, which is just made of awesomeness.

In short this is just a glue between flate2 and clap ;P
