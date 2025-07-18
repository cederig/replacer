use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::time::Instant;
use encoding_rs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the file to modify
    #[arg(short, long)]
    file: String,

    /// The string to search for
    #[arg(short, long)]
    old: String,

    /// The string to replace with
    #[arg(short, long)]
    new: String,

    /// Display statistics about the replacement (number of replacements, time taken)
    #[arg(long)]
    stat: bool,

    /// Path to the output file. If not specified, the original file will be modified in place.
    #[arg(short = 'w', long)]
    output: Option<String>,

    /// Specify the encoding of the input file (e.g., UTF-8, Latin-1, Shift_JIS). If not specified, attempts auto-detection.
    #[arg(short, long)]
    encoding: Option<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let start_time = Instant::now();

    let mut file = fs::File::open(&args.file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let (encoding, had_bom) = if let Some(enc_label) = &args.encoding {
        // User specified encoding
        (encoding_rs::Encoding::for_label(enc_label.as_bytes()).unwrap_or_else(|| {
            eprintln!("Warning: Unknown encoding label '{}'. Defaulting to UTF-8.", enc_label);
            encoding_rs::UTF_8
        }), false)
    } else {
        // Auto-detect encoding
        if let Some((enc, _confidence)) = encoding_rs::Encoding::for_bom(&buffer) {
            (enc, true)
        } else {
            // No BOM, try UTF-8 first
            let (_cow, _, utf8_had_errors) = encoding_rs::UTF_8.decode(&buffer);
            if !utf8_had_errors {
                (encoding_rs::UTF_8, false)
            } else {
                // UTF-8 failed, fall back to WINDOWS_1252 as a common default for non-UTF-8 text
                eprintln!("Warning: Input file is not valid UTF-8. Attempting to decode as Windows-1252.");
                (encoding_rs::WINDOWS_1252, false)
            }
        }
    };

    let (cow, _, had_errors) = encoding.decode(&buffer);

    if had_errors && !had_bom {
        eprintln!("Warning: Some characters could not be decoded from the detected/specified encoding. They might be replaced with U+FFFD (replacement character).");
    }

    let contents = cow.into_owned();

    let occurrences = contents.matches(&args.old).count();
    let replaced_contents = contents.replace(&args.old, &args.new);

    let output_path = if let Some(out_file) = &args.output {
        out_file
    } else {
        &args.file
    };

    let mut file = fs::File::create(output_path)?;
    file.write_all(replaced_contents.as_bytes())?;

    let elapsed_time = start_time.elapsed();

    println!("Successfully replaced all occurrences of '{}' with '{}' in '{}'.", args.old, args.new, args.file);

    if args.stat {
        println!("\n--- Statistics ---");
        println!("Replacements made: {}", occurrences);
        println!("Time taken: {:.2?} ", elapsed_time);
        println!("------------------");
    }

    Ok(())
}