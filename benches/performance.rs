use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use replacer::{perform_replacement, perform_replacement_parallel, perform_replacement_utf8_optimized, perform_multi_pattern_replacement};

fn bench_replacement(c: &mut Criterion) {
    let content = "test ".repeat(100000);
    let pattern = "test";
    let replacement = "TEST";
    
    // Optimized sequential
    c.bench_function("replace_100k_sequential_optimized", |b| {
        b.iter(|| {
            let (_, count) = perform_replacement(
                black_box(&content),
                black_box(pattern),
                black_box(replacement)
            );
            black_box(count);
        });
    });
    
    // Parallel processing
    c.bench_function("replace_100k_parallel", |b| {
        b.iter(|| {
            let (_, count) = perform_replacement_parallel(
                black_box(&content),
                black_box(pattern),
                black_box(replacement),
                black_box(65536)
            );
            black_box(count);
        });
    });
    
    // UTF-8 optimized
    c.bench_function("replace_100k_utf8_optimized", |b| {
        b.iter(|| {
            let (_, count) = perform_replacement_utf8_optimized(
                black_box(&content),
                black_box(pattern),
                black_box(replacement),
                black_box(false)
            );
            black_box(count);
        });
    });
    
    // With caching
    c.bench_function("replace_100k_with_cache", |b| {
        b.iter(|| {
            let (_, count) = perform_replacement_utf8_optimized(
                black_box(&content),
                black_box(pattern),
                black_box(replacement),
                black_box(true)
            );
            black_box(count);
        });
    });
    
    // No matches test
    let content_no_match = "hello world ".repeat(100000);
    
    c.bench_function("replace_no_matches_optimized", |b| {
        b.iter(|| {
            let (_, count) = perform_replacement_utf8_optimized(
                black_box(&content_no_match),
                black_box(pattern),
                black_box(replacement),
                black_box(false)
            );
            black_box(count);
        });
    });
    
    // Large pattern test
    let large_pattern = "this_is_a_very_long_pattern_string_that_should_be_slower_to_match";
    let content_large = format!("{} ", large_pattern).repeat(10000);
    
    c.bench_function("replace_large_pattern_optimized", |b| {
        b.iter(|| {
            let (_, count) = perform_replacement_utf8_optimized(
                black_box(&content_large),
                black_box(&large_pattern),
                black_box("replacement"),
                black_box(false)
            );
            black_box(count);
        });
    });
    
    // Multi-pattern replacement
    c.bench_function("replace_multi_pattern", |b| {
        b.iter(|| {
            let patterns = &["apple", "banana", "cherry"];
            let replacements = &["fruit", "yellow", "red"];
            let (_, count) = perform_multi_pattern_replacement(
                black_box(&content),
                black_box(patterns),
                black_box(replacements)
            );
            black_box(count);
        });
    });
    
    // Unicode content test
    let unicode_content = "héllo wörld héllo wörld ".repeat(50000);
    let unicode_pattern = "héllo";
    let unicode_replacement = "hello";
    
    c.bench_function("replace_unicode_content", |b| {
        b.iter(|| {
            let (_, count) = perform_replacement_utf8_optimized(
                black_box(&unicode_content),
                black_box(unicode_pattern),
                black_box(unicode_replacement),
                black_box(false)
            );
            black_box(count);
        });
    });
}

fn bench_parallel_scaling(c: &mut Criterion) {
    let sizes = vec![1000, 10000, 100000, 1000000];
    
    for size in sizes {
        let content = "test ".repeat(size);
        
        c.bench_function(&format!("parallel_{}_replacements", size), |b| {
            b.iter(|| {
                let (_, count) = perform_replacement_parallel(
                    black_box(&content),
                    black_box("test"),
                    black_box("TEST"),
                    black_box(65536)
                );
                black_box(count);
            });
        });
        
        c.bench_function(&format!("sequential_{}_replacements", size), |b| {
            b.iter(|| {
                let (_, count) = perform_replacement(
                    black_box(&content),
                    black_box("test"),
                    black_box("TEST")
                );
                black_box(count);
            });
        });
    }
}

criterion_group!(benches, bench_replacement, bench_parallel_scaling);
criterion_main!(benches);
