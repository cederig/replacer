use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use encoding_rs::Encoding;
use crate::core::{ReplacementConfig, perform_replacement_parallel, perform_replacement_utf8_optimized};

/// Process file with optimized buffering and automatic algorithm selection
pub fn process_file_buffered(
    input_path: &Path,
    output_path: Option<&Path>,
    pattern: &str,
    replacement: &str,
    encoding: &'static Encoding,
    config: &ReplacementConfig
) -> io::Result<(usize, std::time::Duration)> {
    let start_time = std::time::Instant::now();
    
    // Open input file with optimized buffer
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::with_capacity(config.buffer_size, input_file);
    
    // Read entire content for files smaller than threshold
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    
    // Decode content
    let (content, _, _) = encoding.decode(&buffer);
    let content_str = content.into_owned();
    
    // Choose optimal processing strategy
    let (processed_content, replacement_count) = if content_str.len() > config.parallel_threshold {
        perform_replacement_parallel(&content_str, pattern, replacement, config.parallel_threshold / 2)
    } else {
        perform_replacement_utf8_optimized(&content_str, pattern, replacement, config.enable_caching)
    };
    
    // Write output
    let output_path = output_path.unwrap_or(input_path);
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::with_capacity(config.buffer_size, output_file);
    
    // Encode and write
    let (encoded_content, _, _) = encoding.encode(&processed_content);
    writer.write_all(&encoded_content)?;
    writer.flush()?;
    
    let elapsed = start_time.elapsed();
    Ok((replacement_count, elapsed))
}

/// Process multiple files in parallel
pub fn process_files_parallel(
    file_paths: &[&Path],
    pattern: &str,
    replacement: &str,
    encoding: &'static Encoding,
    config: &ReplacementConfig
) -> io::Result<Vec<(String, usize, std::time::Duration)>> {
    use rayon::prelude::*;
    
    file_paths
        .par_iter()
        .map(|&path| {
            let result = process_file_buffered(path, None, pattern, replacement, encoding, config)?;
            Ok((path.display().to_string(), result.0, result.1))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_buffered_processing() {
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(b"test test test").unwrap();
        temp_file.flush().unwrap();
        
        let config = ReplacementConfig::new();
        let result = process_file_buffered(
            temp_file.path(),
            None,
            "test",
            "TEST",
            encoding_rs::UTF_8,
            &config
        ).unwrap();
        
        assert_eq!(result.0, 3);
        
        // Verify file content
        let mut content = String::new();
        let mut file = File::open(temp_file.path()).unwrap();
        file.read_to_string(&mut content).unwrap();
        assert_eq!(content, "TEST TEST TEST");
    }
}
