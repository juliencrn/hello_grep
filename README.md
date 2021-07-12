### ⚠️ W.I.P.
<br /> 

![alt](./assets/screenshot.png)

# Hello, Grep!

This personal learning project is just another grep clone, written in Rust.

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

This is a rust project, so you need have rust working on your machine. 

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/juliencrn/hello_grep.git
   cd hello_grep 
   ```
2. Compile and run
   ```sh
   cargo run -- -h  # To have documentation
   cargo test       # Or Run tests
   cargo run -- -i -n TODO ~/path/to/file # Or exec as grep
   cargo build --release # Production build
   ```



<!-- USAGE EXAMPLES -->
## Usage

```sh
hello_grep 0.1.0

USAGE:
    mini_grep [FLAGS] <pattern> [path]...

FLAGS:
    -i               Make search case insensitive
        --color      Activate color in output
    -h, --help       Prints help information
    -n               Show line number
    -V, --version    Prints version information

ARGS:
    <pattern>
    <path>...
```

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgements

* [Rust Book](https://doc.rust-lang.org/book/)
