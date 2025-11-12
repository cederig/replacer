use crate::core::sequential::{perform_replacement, perform_replacement_ascii};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use aho_corasick::AhoCorasick;

/// Cache for repeated operations
static REPLACEMENT_CACHE: Lazy<std::sync::Mutex<HashMap<String, (String, usize)>>> = 
    Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

/// UTF-8 optimized replacement that chooses the best algorithm based on content characteristics
pub fn perform_replacement_utf8_optimized(content: &str, pattern: &str, replacement: &str, use_cache: bool) -> (String, usize) {
    if use_cache {
        let cache_key = generate_cache_key(content, pattern, replacement);
        
        if let Ok(cache) = REPLACEMENT_CACHE.try_lock() {
            if let Some(cached_result) = cache.get(&cache_key) {
                return cached_result.clone();
            }
        }
    }
    
    let result = if should_use_ascii_optimization(content, pattern, replacement) {
        // Use ASCII-only optimization for better performance
        let content_bytes = content.as_bytes();
        let (result_bytes, count) = perform_replacement_ascii(
            content_bytes, 
            pattern.as_bytes(), 
            replacement.as_bytes()
        );
        // Safe conversion since we verified ASCII-only content
        (String::from_utf8(result_bytes).unwrap(), count)
    } else {
        // Use Unicode-aware replacement
        perform_replacement(content, pattern, replacement)
    };
    
    if use_cache {
        if let Ok(mut cache) = REPLACEMENT_CACHE.try_lock() {
            let cache_key = generate_cache_key(content, pattern, replacement);
            cache.insert(cache_key, result.clone());
        }
    }
    
    result
}

/// Multi-pattern replacement using Aho-Corasick algorithm
pub fn perform_multi_pattern_replacement(content: &str, patterns: &[&str], replacements: &[&str]) -> (String, usize) {
    if patterns.is_empty() || patterns.len() != replacements.len() {
        return (content.to_string(), 0);
    }
    
    let ac = match AhoCorasick::new(patterns) {
        Ok(ac) => ac,
        Err(_) => return (content.to_string(), 0),
    };
    
    let mut result = String::with_capacity(content.len());
    let mut last_end = 0;
    let mut count = 0;
    
    for match_ in ac.find_iter(content) {
        result.push_str(&content[last_end..match_.start()]);
        result.push_str(&replacements[match_.pattern()]);
        last_end = match_.end();
        count += 1;
    }
    
    result.push_str(&content[last_end..]);
    (result, count)
}

/// Streaming replacement for very large files to minimize memory usage
pub fn perform_streaming_replacement(
    input: &mut impl std::io::Read,
    output: &mut impl std::io::Write,
    pattern: &str,
    replacement: &str,
    buffer_size: usize
) -> std::io::Result<usize> {
    if pattern.is_empty() {
        // Just copy input to output if pattern is empty
        std::io::copy(input, output)?;
        return Ok(0);
    }
    
    let mut buffer = vec![0u8; buffer_size];
    let mut overlap = Vec::new();
    let mut total_count = 0;
    
    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        
        let chunk = &buffer[..bytes_read];
        let mut process_chunk = overlap.clone();
        process_chunk.extend_from_slice(chunk);
        
        // Process the chunk, keeping overlap for pattern boundaries
        let overlap_size = pattern.len().saturating_sub(1);
        
        // Only split if we have enough data
        if process_chunk.len() <= overlap_size {
            overlap = process_chunk;
            continue;
        }
        
        let split_point = process_chunk.len().saturating_sub(overlap_size);
        let (to_process, new_overlap) = process_chunk.split_at(split_point);
        overlap = new_overlap.to_vec();
        
        // Convert to string for processing (assuming UTF-8 content)
        if let Ok(chunk_str) = std::str::from_utf8(to_process) {
            let (replaced, count) = super::sequential::perform_replacement(chunk_str, pattern, replacement);
            output.write_all(replaced.as_bytes())?;
            total_count += count;
        } else {
            // Fallback: write original bytes if not valid UTF-8
            output.write_all(to_process)?;
        }
    }
    
    // Process remaining overlap
    if !overlap.is_empty() {
        if let Ok(chunk_str) = std::str::from_utf8(&overlap) {
            let (replaced, count) = super::sequential::perform_replacement(chunk_str, pattern, replacement);
            output.write_all(replaced.as_bytes())?;
            total_count += count;
        } else {
            output.write_all(&overlap)?;
        }
    }
    
    Ok(total_count)
}

fn should_use_ascii_optimization(content: &str, pattern: &str, replacement: &str) -> bool {
    content.is_ascii() && pattern.is_ascii() && replacement.is_ascii()
}

fn generate_cache_key(content: &str, pattern: &str, replacement: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    content.as_bytes().hash(&mut hasher);
    pattern.as_bytes().hash(&mut hasher);
    replacement.as_bytes().hash(&mut hasher);
    
    format!("{}:{}:{}", content.len(), pattern.len(), hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_optimized_ascii() {
        let content = "test test test";
        let pattern = "test";
        let replacement = "TEST";
        
        let (result_optimized, count_optimized) = perform_replacement_utf8_optimized(content, pattern, replacement, false);
        let (result_sequential, count_sequential) = perform_replacement(content, pattern, replacement);
        
        assert_eq!(result_optimized, result_sequential);
        assert_eq!(count_optimized, count_sequential);
        assert_eq!(count_optimized, 3);
    }

    #[test]
    fn test_utf8_optimized_unicode() {
        let content = "héllo héllo héllo";
        let pattern = "héllo";
        let replacement = "hello";
        
        let (result_optimized, count_optimized) = perform_replacement_utf8_optimized(content, pattern, replacement, false);
        let (result_sequential, count_sequential) = perform_replacement(content, pattern, replacement);
        
        assert_eq!(result_optimized, result_sequential);
        assert_eq!(count_optimized, count_sequential);
        assert_eq!(count_optimized, 3);
    }

    #[test]
    fn test_multi_pattern_replacement() {
        let content = "apple banana cherry apple";
        let patterns = &["apple", "banana"];
        let replacements = &["fruit", "yellow"];
        
        let (result, count) = perform_multi_pattern_replacement(content, patterns, replacements);
        
        assert_eq!(result, "fruit yellow cherry fruit");
        assert_eq!(count, 3);
    }

    #[test]
    fn test_caching() {
        let content = "test test test";
        let pattern = "test";
        let replacement = "TEST";
        
        // First call (no cache)
        let (result1, count1) = perform_replacement_utf8_optimized(content, pattern, replacement, true);
        
        // Second call (should use cache)
        let (result2, count2) = perform_replacement_utf8_optimized(content, pattern, replacement, true);
        
        assert_eq!(result1, result2);
        assert_eq!(count1, count2);
        assert_eq!(count1, 3);
    }
}
