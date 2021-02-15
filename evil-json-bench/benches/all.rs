use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use evil_json_bench::{CitmCatalog, Twitter};
use std::io;
use std::path::Path;

fn get_json(name: &str) -> io::Result<String> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join(format!("{}.json", name));
    std::fs::read_to_string(path)
}

fn citm_catalog(c: &mut Criterion) {
    let citm_catalog_str =
        get_json("citm_catalog").expect("Failed to read citm_catalog.json");

    let mut group = c.benchmark_group("citm_catalog.json");
    group.throughput(Throughput::Bytes(citm_catalog_str.len() as u64));

    let citm_catalog: CitmCatalog =
        serde_json::from_str(citm_catalog_str.as_str()).unwrap();

    group.throughput(Throughput::Bytes(
        serde_json::to_string(&citm_catalog).unwrap().len() as u64
    ));
    group.bench_function("serde-json", |b| {
        b.iter(|| serde_json::to_string(&citm_catalog))
    });

    group.throughput(Throughput::Bytes(
        simd_json::to_string(&citm_catalog).unwrap().len() as u64
    ));
    group.bench_function("simd-json", |b| {
        b.iter(|| simd_json::to_string(&citm_catalog))
    });

    group.throughput(Throughput::Bytes(
        evil_json::to_string(&citm_catalog).unwrap().len() as u64
    ));
    group.bench_function("evil-json", |b| {
        b.iter(|| evil_json::to_string(&citm_catalog))
    });

    group.finish();
}

fn twitter(c: &mut Criterion) {
    let twitter_str = get_json("twitter").expect("Failed to read twitter.json");

    let mut group = c.benchmark_group("twitter.json");
    group.throughput(Throughput::Bytes(twitter_str.len() as u64));

    let twitter: Twitter = serde_json::from_str(twitter_str.as_str()).unwrap();

    group.throughput(Throughput::Bytes(
        serde_json::to_string(&twitter).unwrap().len() as u64
    ));
    group.bench_function("serde-json", |b| {
        b.iter(|| serde_json::to_string(&twitter))
    });

    group.throughput(Throughput::Bytes(
        simd_json::to_string(&twitter).unwrap().len() as u64
    ));
    group.bench_function("simd-json", |b| {
        b.iter(|| simd_json::to_string(&twitter))
    });

    group.throughput(Throughput::Bytes(
        evil_json::to_string(&twitter).unwrap().len() as u64
    ));
    group.bench_function("evil-json", |b| {
        b.iter(|| evil_json::to_string(&twitter))
    });

    group.finish();
}

criterion_group!(benches, citm_catalog, twitter);
criterion_main!(benches);
