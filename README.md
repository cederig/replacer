# Replacer

`replacer` is an extremely fast command-line utility written in Rust that allows you to replace all occurrences of a given string with another string in a specified file.

## Dependencies

This project uses the following dependencies (as defined in `Cargo.toml`):

-   `clap` (version `4.5.41`): For command-line argument parsing.
-   `indicatif` (version `0.18.0`): For displaying a progress bar.
-   `encoding_rs` (version `0.8.35`): For handling text encodings.

## Installation

### Prerequisites

Make sure you have Rust and Cargo installed on your system. You can install them by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Compiling for Linux (from Linux/macOS)
1.  Clone this repository:
    ```sh
    git clone https://github.com/cederig/replacer.git
    cd replacer
    ```
2.  Compile the project:
    ```sh
    cargo build --release
    ```
    The executable will be located in `target/release/replacer`.

### Compiling for Windows (from Linux/macOS)

To compile this project for Windows from another operating system (like Linux or macOS), you can use cross-compilation. You will need the Rust target for Windows.

1.  Add the Windows target to your Rust installation:
    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```

2.  Compile the project for the Windows target:
    ```sh
    cargo build --release --target=x86_64-pc-windows-gnu
    ```

The Windows executable will be located in `target/x86_64-pc-windows-gnu/release/replacer.exe`.

### Compiling for macOS (from Linux/macOS)

To compile this project for macOS from another operating system (like Linux or macOS), you can use cross-compilation. You will need the Rust target for macOS.

1.  Add the macOS target to your Rust installation (choose the correct architecture):
    *   For Intel Macs (x86_64):
        ```sh
        rustup target add x86_64-apple-darwin
        ```
    *   For Apple Silicon Macs (aarch64):
        ```sh
        rustup target add aarch64-apple-darwin
        ```

2.  Compile the project for the macOS target (choose the correct architecture):
    *   For Intel Macs:
        ```sh
        cargo build --release --target=x86_64-apple-darwin
        ```
    *   For Apple Silicon Macs:
        ```sh
        cargo build --release --target=aarch64-apple-darwin
        ```

The macOS executable will be located in `target/<your_mac_target>/release/replacer` (e.g., `target/x86_64-apple-darwin/release/replacer`).

## Usage

The basic syntax is as follows:

```bash
./replacer [OPTIONS] --file <FILE> --old <OLD> --new <NEW>
```

### Options

*   `-f`, `--file <file_path>`: Specifies the path to the file to read. (Required)
*   `-o`, `--old <old_string>`: The string to search for and replace. (Required)
*   `-n`, `--new <new_string>`: The string to replace the old string with. (Required)
*   `--stat`: Displays statistics about the replacement, including the number of replacements made and processing time. (Optional)
*   `-w`, `--output <output_file_path>`: Specifies an output file. If this option is used, the source file will not be modified, and the replaced content will be written to this new file. (Optional)
*   `-e`, `--encoding <encoding>`: Specifies the encoding of the input file (e.g., `UTF-8`, `Latin-1`, `Shift_JIS`). If this option is not specified, the program will attempt to automatically detect the encoding (priority to BOM, then UTF-8, then Windows-1252 as a last resort). (Optional)



### Examples

1.  Replace "Bonjour" with "Salut" in `example.txt` (automatic encoding detection) and display statistics:

    ```bash
    ./replacer -f example.txt -o "Bonjour" -n "Salut" --stat
    ```

2.  Replace all occurrences of "erreur" with "succès" in `log.txt` (encoded in Latin-1) and write the result to `log_modified.txt`:

    ```bash
    ./replacer -f log.txt -o "erreur" -n "succès" -w log_modified.txt -e Latin-1
    ```

3.  Replace "pomme" with "orange" in `fruits.txt` (automatic encoding detection), write the result to `new_fruits.txt` and display statistics:

    ```bash
    ./replacer -f fruits.txt -o "pomme" -n "orange" -w new_fruits.txt --stat
    ```

## Performance

`replacer` is designed to be extremely fast. Thanks to Rust's efficiency, it is capable of processing large files and performing a significant number of replacements in record time. For example, it can replace 50,000 occurrences of a string in less than 50 milliseconds on typical hardware configurations.

## Tests

This project includes unit tests; to run them, use the following command at the root of the project:

```bash
cargo test
```

This command compiles the program in test mode and runs all test functions.

