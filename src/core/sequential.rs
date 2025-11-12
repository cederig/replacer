/// Optimized replacement function that counts and replaces in a single pass
pub fn perform_replacement(content: &str, pattern: &str, replacement: &str) -> (String, usize) {
    if pattern.is_empty() {
        return (content.to_string(), 0);
    }
    
    let mut result = String::with_capacity(content.len());
    let mut count = 0;
    let mut last_end = 0;
    
    for (start, _part) in content.match_indices(pattern) {
        result.push_str(&content[last_end..start]);
        result.push_str(replacement);
        last_end = start + pattern.len();
        count += 1;
    }
    
    result.push_str(&content[last_end..]);
    (result, count)
}

/// Alternative implementation using memchr for byte-based optimization (ASCII only)
pub fn perform_replacement_ascii(content: &[u8], pattern: &[u8], replacement: &[u8]) -> (Vec<u8>, usize) {
    if pattern.is_empty() {
        return (content.to_vec(), 0);
    }
    
    let mut result = Vec::with_capacity(content.len());
    let mut count = 0;
    let mut last_end = 0;
    
    let mut pos = 0;
    while pos <= content.len().saturating_sub(pattern.len()) {
        if &content[pos..pos + pattern.len()] == pattern {
            result.extend_from_slice(&content[last_end..pos]);
            result.extend_from_slice(replacement);
            pos += pattern.len();
            last_end = pos;
            count += 1;
        } else {
            pos += 1;
        }
    }
    
    result.extend_from_slice(&content[last_end..]);
    (result, count)
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
    fn test_empty_pattern() {
        let content = "Hello world!";
        let pattern = "";
        let replacement = "test";
        let (replaced_content, count) = perform_replacement(content, pattern, replacement);
        assert_eq!(replaced_content, "Hello world!");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_ascii_optimization() {
        let content = b"test test test";
        let pattern = b"test";
        let replacement = b"TEST";
        let (result, count) = perform_replacement_ascii(content, pattern, replacement);
        assert_eq!(result, b"TEST TEST TEST");
        assert_eq!(count, 3);
    }
}
