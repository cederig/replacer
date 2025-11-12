#[derive(Debug, Clone)]
pub struct ReplacementConfig {
    pub parallel_threshold: usize,
    pub buffer_size: usize,
    pub use_ascii_optimization: bool,
    pub enable_caching: bool,
}

impl Default for ReplacementConfig {
    fn default() -> Self {
        Self {
            parallel_threshold: 1024 * 1024, // 1MB
            buffer_size: 8 * 1024 * 1024,    // 8MB
            use_ascii_optimization: true,
            enable_caching: false,
        }
    }
}

impl ReplacementConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_parallel_threshold(mut self, threshold: usize) -> Self {
        self.parallel_threshold = threshold;
        self
    }
    
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
    
    pub fn with_ascii_optimization(mut self, enabled: bool) -> Self {
        self.use_ascii_optimization = enabled;
        self
    }
    
    pub fn with_caching(mut self, enabled: bool) -> Self {
        self.enable_caching = enabled;
        self
    }
}
