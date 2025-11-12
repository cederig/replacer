use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::core::sequential::{perform_replacement_ascii, perform_replacement};

/// Parallel replacement for large files
pub fn perform_replacement_parallel(content: &str, pattern: &str, replacement: &str, chunk_size: usize) -> (String, usize) {
    if content.len() < chunk_size {
        return perform_replacement(content, pattern, replacement);
    }
    
    // For ASCII content, use byte-based parallel processing
    if content.is_ascii() && pattern.is_ascii() && replacement.is_ascii() {
        return perform_replacement_parallel_ascii(content.as_bytes(), pattern.as_bytes(), replacement.as_bytes(), chunk_size);
    }
    
    // Unicode parallel processing (more complex due to UTF-8 boundaries)
    perform_replacement_parallel_unicode(content, pattern, replacement, chunk_size)
}

fn perform_replacement_parallel_unicode(content: &str, pattern: &str, replacement: &str, chunk_size: usize) -> (String, usize) {
    let total_count = AtomicUsize::new(0);
    
    // Find safe UTF-8 boundaries for chunking
    let chunks: Vec<_> = find_utf8_chunks(content, chunk_size)
        .par_iter()
        .map(|chunk| {
            let (result, count) = perform_replacement(chunk, pattern, replacement);
            total_count.fetch_add(count, Ordering::Relaxed);
            result
        })
        .collect();
    
    (chunks.join(""), total_count.load(Ordering::Relaxed))
}

fn perform_replacement_parallel_ascii(content: &[u8], pattern: &[u8], replacement: &[u8], chunk_size: usize) -> (String, usize) {
    let total_count = AtomicUsize::new(0);
    
    // Split into chunks, ensuring we don't split in the middle of potential matches
    let safe_chunk_size = chunk_size.max(pattern.len() * 2);
    let chunks: Vec<_> = content.chunks(safe_chunk_size)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|chunk| {
            let (result, count) = perform_replacement_ascii(chunk, pattern, replacement);
            total_count.fetch_add(count, Ordering::Relaxed);
            result
        })
        .collect();
    
    let combined: Vec<u8> = chunks.into_iter().flatten().collect();
    let total_replacements = total_count.load(Ordering::Relaxed);
    
    // Safe conversion since we know all content is valid ASCII/UTF-8
    (String::from_utf8(combined).unwrap(), total_replacements)
}

fn find_utf8_chunks(content: &str, chunk_size: usize) -> Vec<&str> {
    let mut chunks = Vec::new();
    let mut start = 0;
    
    while start < content.len() {
        let end = (start + chunk_size).min(content.len());
        
        // Find a safe UTF-8 boundary (character boundary)
        let safe_end = if end < content.len() {
            content.ceil_char_boundary(end)
        } else {
            end
        };
        
        chunks.push(&content[start..safe_end]);
        start = safe_end;
    }
    
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::sequential::perform_replacement;

    #[test]
    fn test_parallel_ascii_replacement() {
        let content = "test ".repeat(100);
        let pattern = "test";
        let replacement = "TEST";
        
        let (result_parallel, count_parallel) = perform_replacement_parallel(&content, pattern, replacement, 1024);
        let (result_sequential, count_sequential) = perform_replacement(&content, pattern, replacement);
        
        println!("Parallel result length: {}", result_parallel.len());
        println!("Sequential result length: {}", result_sequential.len());
        println!("Parallel count: {}", count_parallel);
        println!("Sequential count: {}", count_sequential);
        
        assert_eq!(result_parallel, result_sequential);
        assert_eq!(count_parallel, count_sequential);
        assert_eq!(count_parallel, 100);
    }

    #[test]
    fn test_parallel_small_content() {
        let content = "Hello world!";
        let pattern = "world";
        let replacement = "Rust";
        
        let (result_parallel, count_parallel) = perform_replacement_parallel(&content, pattern, replacement, 1024);
        let (result_sequential, count_sequential) = perform_replacement(&content, pattern, replacement);
        
        assert_eq!(result_parallel, result_sequential);
        assert_eq!(count_parallel, count_sequential);
    }
}
