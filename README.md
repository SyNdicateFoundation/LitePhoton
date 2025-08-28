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
git clone https://github.com/your-username/LitePhoton.git
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
<img src="https://cdn.discordapp.com/attachments/1406334294875570219/1410668503459172372/5JwqH5P.png?ex=68b1daec&is=68b0896c&hm=b13036dfaafc67e5993fe52d24918c2e22fcd2498904a28222806a0fb4be523c&"/>


## Contribution Guidelines 🤝
Feel free to contribute to the development of our bot. we will notice it.