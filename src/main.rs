use clap::Parser;
use std::fs;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};
use encoding_rs;
use indicatif::{ProgressBar, ProgressStyle};

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
}

// This function contains the core replacement logic and is now testable.
fn perform_replacement(content: &str, pattern: &str, replacement: &str) -> (String, usize) {
    let occurrences = content.matches(pattern).count();
    let replaced_contents = content.replace(pattern, replacement);
    (replaced_contents, occurrences)
}


fn main() -> io::Result<()> {
    let args = Args::parse();

    let start_time = Instant::now();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ])
            .template("{spinner:.blue} {msg}")
            .unwrap(),
    );
    pb.set_message("Reading file...");

    let mut file = fs::File::open(&args.file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    pb.set_message("Decoding file...");

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

    pb.set_message("Replacing content...");
    let (replaced_contents, occurrences) = perform_replacement(&contents, &args.pattern, &args.replacement);

    pb.set_message("Writing to file...");
    let output_path = if let Some(out_file) = &args.output {
        out_file
    } else {
        &args.file
    };

    let (encoded_output, _, had_encoding_errors) = encoding.encode(&replaced_contents);
    if had_encoding_errors {
        eprintln!("Warning: Some characters in the replacement string could not be represented in the target encoding ('{}'). They may have been replaced by fallback characters.", encoding.name());
    }

    let mut file = fs::File::create(output_path)?;

    // If the original file had a BOM, write a BOM to the output file.
    if had_bom {
        match encoding.name() {
            "UTF-8" => file.write_all(&[0xEF, 0xBB, 0xBF])?,
            "UTF-16LE" => file.write_all(&[0xFF, 0xFE])?,
            "UTF-16BE" => file.write_all(&[0xFE, 0xFF])?,
            _ => {} // Not all encodings have a BOM
        }
    }

    file.write_all(&encoded_output)?;

    pb.finish_and_clear();

    let elapsed_time = start_time.elapsed();

        if args.stat {
        println!("Successfully replaced all occurrences of '{}' with '{}' in '{}'.", args.pattern, args.replacement, args.file);
        println!("----- Statistics -----");
        println!("Replacements made: {}", occurrences);
        println!("Time taken: {:.2?} ", elapsed_time);
        println!("----------------------");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_replacement() {
        let content = "Hello world, world!";
        let pattern = "world";
        let replacement = "Rust";
        let (replaced_content, count) = perform_replacement(content, pattern, replacement);
        assert_eq!(replaced_content, "Hello Rust, Rust!");
        assert_eq!(count, 2);
    }

    #[test]
    fn test_no_match() {
        let content = "Hello world!";
        let pattern = "galaxy";
        let replacement = "Rust";
        let (replaced_content, count) = perform_replacement(content, pattern, replacement);
        assert_eq!(replaced_content, "Hello world!");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_case_sensitive() {
        let content = "Hello World!";
        let pattern = "world";
        let replacement = "Rust";
        let (replaced_content, count) = perform_replacement(content, pattern, replacement);
        assert_eq!(replaced_content, "Hello World!");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_empty_content() {
        let content = "";
        let pattern = "a";
        let replacement = "b";
        let (replaced_content, count) = perform_replacement(content, pattern, replacement);
        assert_eq!(replaced_content, "");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_empty_replacement_string() {
        let content = "Hello world!";
        let pattern = "world";
        let replacement = "";
        let (replaced_content, count) = perform_replacement(content, pattern, replacement);
        assert_eq!(replaced_content, "Hello !");
        assert_eq!(count, 1);
    }
}