pub mod sequential;
pub mod parallel;
pub mod specialized;
pub mod config;

pub use sequential::perform_replacement;
pub use parallel::perform_replacement_parallel;
pub use specialized::{
    perform_replacement_utf8_optimized, 
    perform_multi_pattern_replacement,
    perform_streaming_replacement
};
pub use config::ReplacementConfig;
