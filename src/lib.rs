pub mod core;
pub mod io;

pub use core::{
    ReplacementConfig, 
    perform_replacement, 
    perform_replacement_parallel, 
    perform_replacement_utf8_optimized,
    perform_multi_pattern_replacement,
    perform_streaming_replacement
};
pub use io::{
    process_file_buffered,
    process_file_streaming
};

use std::fs;
use std::io::{self as stdio, Read, Write};
use std::time::Instant;
use encoding_rs;
use indicatif::{ProgressBar, ProgressStyle};

/// Main high-level replacement function with automatic optimization
pub fn replace_in_file(
    file_path: &str,
    pattern: &str,
    replacement: &str,
    output_file: Option<&str>,
    encoding: Option<&str>,
    show_stats: bool,
    config: Option<ReplacementConfig>
) -> stdio::Result<()> {
    let args = Args {
        file: file_path.to_string(),
        pattern: pattern.to_string(),
        replacement: replacement.to_string(),
        stat: show_stats,
        output: output_file.map(|s| s.to_string()),
        encoding: encoding.map(|s| s.to_string()),
    };
    
    run_replacement(args, config.unwrap_or_default())
}

#[derive(Debug)]
struct Args {
    file: String,
    pattern: String,
    replacement: String,
    stat: bool,
    output: Option<String>,
    encoding: Option<String>,
}

fn run_replacement(args: Args, config: ReplacementConfig) -> stdio::Result<()> {
    let start_time = Instant::now();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
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
        (encoding_rs::Encoding::for_label(enc_label.as_bytes()).unwrap_or_else(|| {
            eprintln!("Warning: Unknown encoding label '{}'. Defaulting to UTF-8.", enc_label);
            encoding_rs::UTF_8
        }), false)
    } else {
        if let Some((enc, _confidence)) = encoding_rs::Encoding::for_bom(&buffer) {
            (enc, true)
        } else {
            let (_cow, _, utf8_had_errors) = encoding_rs::UTF_8.decode(&buffer);
            if !utf8_had_errors {
                (encoding_rs::UTF_8, false)
            } else {
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
    
    // Use optimized processing
    let (replaced_contents, occurrences) = if contents.len() > config.parallel_threshold {
        pb.set_message("Processing with parallel algorithm...");
        perform_replacement_parallel(&contents, &args.pattern, &args.replacement, config.parallel_threshold / 2)
    } else {
        pb.set_message("Processing with optimized algorithm...");
        perform_replacement_utf8_optimized(&contents, &args.pattern, &args.replacement, config.enable_caching)
    };

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

    if had_bom {
        match encoding.name() {
            "UTF-8" => file.write_all(&[0xEF, 0xBB, 0xBF])?,
            "UTF-16LE" => file.write_all(&[0xFF, 0xFE])?,
            "UTF-16BE" => file.write_all(&[0xFE, 0xFF])?,
            _ => {}
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
        println!("Processing method: {}", 
            if contents.len() > config.parallel_threshold { "Parallel" } 
            else { "Optimized Sequential" });
        println!("----------------------");
    }

    Ok(())
}
