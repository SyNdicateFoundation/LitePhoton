# LitePhoton: a blazing fast text file reader/scanner.


## Overview

- this project is designed to be a blazing fast text file scanner/reader.

### Key Features 🚀

- This project is intended to be lightweight and fast as possible.
- it may not contain many features as of now, but ideas are welcomed.

## Installation

To use LitePhoton, you'll need to have Rust installed on your system. You can download and install Rust from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone the LitePhoton repository and build the project:

```
git clone https://github.com/SyNdicateFoundation/LitePhoton.git
cd LitePhoton
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

You can also download binaries available on Github page, without requiring you to do all these.

## Getting Started 🚧

LitePhoton is a command-line tool that can be used to search for a specific keyword in a file. To use it, run the following command:

```
./LitePhoton -f <file_path> -k <keyword>
```

You can also specify the search method using the `-m` option. The available methods are `chunk` and `normal`.

```
./LitePhoton -f <file_path> -k <keyword> -m <method>
```
## A brief comparison
# Test details
- Debian 13
- Intel(R) Xeon(R) E5-2667 v2 (6) @ 3.29 GHz
- 16 GB
<img src="https://media.discordapp.net/attachments/1406334294875570219/1410670184573960262/zaJcqyR.png?ex=68b1dc7c&is=68b08afc&hm=7c52eb368574175fff7186e9fea819f3a3636f33d4a9ed2798b45a81c1c0787a&=&format=webp&quality=lossless&width=1314&height=681"/>


## Contribution Guidelines 🤝
Feel free to contribute to the development of our project. we will notice it.
