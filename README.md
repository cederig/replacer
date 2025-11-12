# Replacer

`replacer` is an ultra-fast command-line tool written in Rust that allows you to replace all occurrences of a given string with another string in specified files. Featuring advanced optimizations, parallel processing, and intelligent algorithm selection.

## Features

`replacer` is designed to be extremely fast with multiple performance optimizations:

- **40-50% faster than naive implementations** through single-pass processing
- **Parallel processing** for large files (>1MB by default) using Rayon
- **ASCII optimization** for ASCII-only content (2-3x faster)
- **Intelligent caching** for repeated operations
- **Memory-efficient streaming** for very large files
- **Multi-pattern support** using Aho-Corasick algorithm
- **Automatic encoding detection** with BOM support
- **UTF-8 aware processing** with safe boundary handling

### Performance Benchmarks

| Test Case | Optimized | Original | Improvement |
|-----------|-----------|----------|-------------|
| 100k replacements | 2.5ms | 4.2ms | **+40%** |
| No matches | 389µs | 775µs | **+50%** |
| Large pattern | 880µs | 1.4ms | **+37%** |
| ASCII optimization | 1.9ms | N/A | **New feature** |

## Dependencies

This project uses the following dependencies (as defined in `Cargo.toml`):

- `clap` (version `4.5.51`): For command-line argument parsing
- `indicatif` (version `0.18.2`): For displaying a progress bar
- `encoding_rs` (version `0.8.35`): For handling text encodings
- `rayon` (version `1.11.0`): For parallel processing
- `aho-corasick` (version `1.1.4`): For multi-pattern replacement
- `once_cell` (version `1.21.3`): For caching mechanism
- `tempfile` (version `3.14.0`): For testing

## Installation

### Prerequisites

Make sure you have Rust and Cargo installed on your system. You can install them by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Compiling for Linux (from Linux)
1. Clone this repository:
    ```sh
    git clone https://github.com/cederig/replacer.git
    cd replacer
    ```
2. Compile the project:
    ```sh
    cargo build --release
    ```
    The executable will be located in `target/release/replacer`.

### Compiling for Windows (from Linux/macOS)

To cross-compile this project for Windows from another operating system (like Linux or macOS), you will need the Rust target for Windows.

1. Add the Windows target to your Rust installation:
    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```

2. Compile the project for the Windows target:
    ```sh
    cargo build --release --target=x86_64-pc-windows-gnu
    ```

The Windows executable will be located in `target/x86_64-pc-windows-gnu/release/replacer.exe`.

### Compiling for macOS (from Linux/macOS)

To cross-compile this project for macOS from another operating system (like Linux or macOS), you will need the Rust target for macOS.

1. Add the macOS target to your Rust installation (choose the correct architecture):
   * For Intel Macs (x86_64):
        ```sh
        rustup target add x86_64-apple-darwin
        ```
   * For Apple Silicon Macs (aarch64):
        ```sh
        rustup target add aarch64-apple-darwin
        ```

2. Compile the project for the macOS target (choose the correct architecture):
   * For Intel Macs:
        ```sh
        cargo build --release --target=x86_64-apple-darwin
        ```
   * For Apple Silicon Macs:
        ```sh
        cargo build --release --target=aarch64-apple-darwin
        ```

The macOS executable will be located in `target/<your_mac_target>/release/replacer`.

## Usage

The basic syntax is as follows:

```sh
./replacer [OPTIONS] --file <FILE> --pattern <PATTERN> --replacement <REPLACEMENT>
```

### Options

- `-f`, `--file <file_path>`: Specifies the path to the file to read. (Required)
- `-p`, `--pattern <pattern_string>`: The string to search for and replace. (Required)
- `-r`, `--replacement <replacement_string>`: The string to replace the pattern string with. (Required)
- `--stat`: Displays statistics about the replacement, including the number of replacements made and processing time. (Optional)
- `-w`, `--output <output_file_path>`: Specifies an output file. If this option is used, the source file will not be modified, and the replaced content will be written to this replacement file. (Optional)
- `-e`, `--encoding <encoding>`: Specifies the encoding of the input file (e.g., `UTF-8`, `Latin-1`, `Shift_JIS`). If this option is not specified, the program will attempt to automatically detect the encoding (priority to BOM, then UTF-8, then Windows-1252 as a last resort). (Optional)
- `--parallel`: Enable parallel processing for large files (default: auto-detect based on file size). (Optional)
- `--no-cache`: Disable caching for repeated operations (default: enabled for small files). (Optional)
- `--ascii-opt`: Force ASCII optimization when possible (default: auto-detect). (Optional)
- `--buffer-size <size>`: Buffer size for file I/O operations (default: 8MB). (Optional)
- `--parallel-threshold <size>`: Threshold for parallel processing (default: 1MB). (Optional)

### Performance Tips

- Use `--parallel` for files larger than 1MB to enable multi-core processing
- Enable `--ascii-opt` when working with ASCII-only text for 2-3x speedup
- Adjust `--buffer-size` based on available memory (larger = faster but more RAM usage)
- Use `--no-cache` for one-time operations to avoid cache overhead

## Examples

- Replace "Bonjour" with "Salut" in `example.txt` (automatic encoding detection) and display statistics:
    ```sh
    ./replacer -f example.txt -p "Bonjour" -r "Salut" --stat
    ```

- Replace all occurrences of "erreur" with "succès" in `log.txt` (encoded in Latin-1) and write the result to `log_modified.txt`:
    ```sh
    ./replacer -f log.txt -p "erreur" -r "succès" -w log_modified.txt -e Latin-1
    ```

- Replace "pomme" with "orange" in `fruits.txt` (automatic encoding detection), write the result to `replacement_fruits.txt` and display statistics:
    ```sh
    ./replacer -f fruits.txt -p "pomme" -r "orange" -w replacement_fruits.txt --stat
    ```

- Process a large file with parallel processing and ASCII optimization:
    ```sh
    ./replacer -f large_file.txt -p "old" -r "new" --parallel --ascii-opt --stat
    ```

- Custom buffer size for memory-constrained environments:
    ```sh
    ./replacer -f huge_file.txt -p "pattern" -r "replacement" --buffer-size 4194304 --stat
    ```

## Advanced Features

### Automatic Algorithm Selection
The tool automatically chooses the best algorithm based on:
- File size (parallel vs sequential)
- Content type (ASCII vs Unicode)
- Pattern characteristics
- Available system memory

### Memory Efficiency
- Streaming processing for files larger than 10x buffer size
- Configurable buffer sizes
- Minimal memory footprint for small files
- UTF-8 boundary-safe chunking

### Multi-Core Processing
- Automatic parallelization for large files
- Safe UTF-8 boundary handling in parallel mode
- Load balancing across CPU cores
- Fallback to sequential processing for small files

## Tests

This project includes comprehensive unit tests and benchmarks:

```sh
# Run unit tests
cargo test

# Run performance benchmarks
cargo bench

# Run tests with output
cargo test -- --nocapture
```

## Architecture

The project uses a modular architecture:

```
src/
├── lib.rs              # Public API and main interface
├── main.rs             # CLI entry point
├── core/               # Core algorithms
│   ├── mod.rs          # Module exports
│   ├── sequential.rs   # Optimized sequential processing
│   ├── parallel.rs     # Parallel processing algorithms
│   ├── specialized.rs  # Specialized optimizations (ASCII, caching, multi-pattern)
│   └── config.rs       # Configuration management
└── io/                 # I/O operations
    ├── mod.rs          # Module exports
    ├── buffered.rs     # Buffered file operations
    └── streaming.rs    # Streaming for very large files
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

