use clap::Parser;
use replacer::{replace_in_file, ReplacementConfig};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to modify
    #[arg(short, long)]
    file: String,

    /// The string to search for
    #[arg(short, long)]
    pattern: String,

    /// The string to replace with
    #[arg(short, long)]
    replacement: String,

    /// Display statistics about the replacement (number of replacements, time taken)
    #[arg(long)]
    stat: bool,

    /// Path to the output file. If not specified, the original file will be modified in place.
    #[arg(short = 'w', long)]
    output: Option<String>,

    /// Specify the encoding of the input file (e.g., UTF-8, Latin-1, Shift_JIS). If not specified, attempts auto-detection.
    #[arg(short, long)]
    encoding: Option<String>,

    /// Enable parallel processing for large files (default: auto-detect based on file size)
    #[arg(long)]
    parallel: bool,

    /// Disable caching for repeated operations (default: enabled for small files)
    #[arg(long)]
    no_cache: bool,

    /// Force ASCII optimization when possible (default: auto-detect)
    #[arg(long)]
    ascii_opt: bool,

    /// Buffer size for file I/O operations (default: 8MB)
    #[arg(long, default_value = "8388608")]
    buffer_size: usize,

    /// Threshold for parallel processing (default: 1MB)
    #[arg(long, default_value = "1048576")]
    parallel_threshold: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Build configuration
    let mut config = ReplacementConfig::new()
        .with_buffer_size(args.buffer_size)
        .with_parallel_threshold(args.parallel_threshold)
        .with_ascii_optimization(args.ascii_opt)
        .with_caching(!args.no_cache);

    // Override parallel setting if explicitly specified
    if args.parallel {
        config.parallel_threshold = 0; // Force parallel processing
    }

    replace_in_file(
        &args.file,
        &args.pattern,
        &args.replacement,
        args.output.as_deref(),
        args.encoding.as_deref(),
        args.stat,
        Some(config),
    )?;

    Ok(())
}
