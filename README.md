# Random Page Server

This project is a Rust web server using the Hyper library. The server responds to HTTP requests and dynamically generates pages with random alphanumeric content.

## Features

- The server listens on a customizable port, defaulting to port 80.
- The server generates pages with random alphanumeric content. The size of the generated text can be adjusted with a command-line parameter.
- The server utilizes a multi-threaded Tokio runtime, optimizing for the number of available CPU cores.
- User-friendly error messages guide the correct usage of command-line parameters.

## Usage

The program requires two command-line parameters in the following order:

```
random-page-server <port> <random_text_size_in_kb>
```

- `port`: The port for the server to listen on.
- `random_text_size_in_kb`: The size of the random alphanumeric text to be generated, in kilobytes.

If the parameters are missing or incorrect, the server will default to port 80 and generate 20KB of random alphanumeric text.

## Building

You need to have Rust and Cargo installed to build this project. Once installed, navigate to the project's root directory in your terminal and run:

```
cargo build --release
```

This will create an executable in the `./target/release/` directory.

## Running

After building, you can run the server with:

```
./target/release/random-page-server <port> <random_text_size_in_kb>
```

## Dependencies

This project depends on the following Rust libraries:

- hyper: For handling HTTP and creating the server.
- rand: For generating random alphanumeric text.
- tokio: For managing async tasks and creating a multi-threaded runtime.
- std: For basic Rust functionality like command-line argument parsing, input/output, and environment interactions.

Please refer to the `Cargo.toml` file for the exact versions of these dependencies.
