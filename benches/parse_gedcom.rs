use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gedcom_rs::parse::{parse_gedcom, GedcomConfig};
use gedcom_rs::types::EventType;

use std::time::Duration;

const FILENAME: &str = "data/complete.ged";

fn parse_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse-gedcom");
    group.measurement_time(Duration::from_secs(30));

    // Benchmark with default configuration (no verbose output)
    let config = GedcomConfig::new();

    // TODO: Benchmark individual types?
    group.bench_function("parse gedcom", |b| {
        b.iter(|| parse_gedcom(FILENAME, &config).expect("Failed to parse GEDCOM"))
    });
    group.finish();
}

fn search_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("search");
    group.measurement_time(Duration::from_secs(5));

    let config = GedcomConfig::new();
    let gedcom = parse_gedcom(FILENAME, &config).expect("Failed to parse GEDCOM");

    group.bench_function("find_individual_by_xref", |b| {
        b.iter(|| gedcom.find_individual_by_xref(black_box("@I1@")))
    });

    group.bench_function("find_individuals_by_name", |b| {
        b.iter(|| gedcom.find_individuals_by_name(black_box("Torture")))
    });

    group.bench_function("find_family_by_xref", |b| {
        b.iter(|| gedcom.find_family_by_xref(black_box("@F1@")))
    });

    group.bench_function("find_individuals_by_event_date", |b| {
        b.iter(|| {
            gedcom.find_individuals_by_event_date(black_box(EventType::Birth), black_box("1965"))
        })
    });

    group.finish();
}

fn basic_relationships_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic-relationships");
    group.measurement_time(Duration::from_secs(5));

    let config = GedcomConfig::new();
    let gedcom = parse_gedcom(FILENAME, &config).expect("Failed to parse GEDCOM");
    let person = gedcom
        .find_individual_by_xref("@I1@")
        .expect("Failed to find person");

    group.bench_function("get_parents", |b| {
        b.iter(|| gedcom.get_parents(black_box(person)))
    });

    group.bench_function("get_children", |b| {
        b.iter(|| gedcom.get_children(black_box(person)))
    });

    group.bench_function("get_spouses", |b| {
        b.iter(|| gedcom.get_spouses(black_box(person)))
    });

    group.bench_function("get_siblings", |b| {
        b.iter(|| gedcom.get_siblings(black_box(person)))
    });

    group.bench_function("get_full_siblings", |b| {
        b.iter(|| gedcom.get_full_siblings(black_box(person)))
    });

    group.bench_function("get_half_siblings", |b| {
        b.iter(|| gedcom.get_half_siblings(black_box(person)))
    });

    group.finish();
}

fn advanced_relationships_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("advanced-relationships");
    group.measurement_time(Duration::from_secs(5));

    let config = GedcomConfig::new();
    let gedcom = parse_gedcom(FILENAME, &config).expect("Failed to parse GEDCOM");
    let person = gedcom
        .find_individual_by_xref("@I1@")
        .expect("Failed to find person");

    group.bench_function("get_ancestors", |b| {
        b.iter(|| gedcom.get_ancestors(black_box(person), black_box(Some(5))))
    });

    group.bench_function("get_descendants", |b| {
        b.iter(|| gedcom.get_descendants(black_box(person), black_box(Some(5))))
    });

    let person2 = gedcom
        .find_individual_by_xref("@I3@")
        .expect("Failed to find person 2");

    group.bench_function("find_relationship_path", |b| {
        b.iter(|| gedcom.find_relationship_path(black_box(person), black_box(person2)))
    });

    group.bench_function("find_relationship", |b| {
        b.iter(|| gedcom.find_relationship(black_box(person), black_box(person2)))
    });

    group.finish();
}

criterion_group!(
    benches,
    parse_benchmark,
    search_benchmark,
    basic_relationships_benchmark,
    advanced_relationships_benchmark
);
criterion_main!(benches);
