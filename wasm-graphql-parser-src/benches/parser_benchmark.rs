use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use wasm_graphql_parser::{parse_schema_from_file_and_print, types::Config};

fn criterion_benchmark(c: &mut Criterion) {
    let parellel_config: Config = Config {
        parallel: true,
        run_in_wasm: false
    };

    let sync_config: Config = Config {
        parallel: false,
        run_in_wasm: false
    };

    let schema_file_path = "./benches/data/schema.graphql";

    
    let mut group = c.benchmark_group("sample-size-example");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.1).sample_size(500);
    group.measurement_time(Duration::new(30, 0));


    group.bench_function("sync_parser", |b| b.iter(|| parse_schema_from_file_and_print(black_box(schema_file_path), black_box(&sync_config))));
    group.bench_function("parallel_parser", |b| b.iter(|| parse_schema_from_file_and_print(black_box(schema_file_path), black_box(&parellel_config))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);