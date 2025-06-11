use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use odds_converter::Odds;

fn benchmark_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("conversions");

    // Test data
    let american_odds = vec![100, 150, -110, -200, 300, -150];
    let decimal_odds = vec![1.5, 2.0, 2.5, 3.0, 4.0, 1.91];
    let fractional_odds = vec![(1, 2), (3, 2), (2, 1), (5, 1), (1, 4), (9, 4)];

    // Benchmark American to Decimal conversion
    for &american in &american_odds {
        group.bench_with_input(
            BenchmarkId::new("american_to_decimal", american),
            &american,
            |b, &american| {
                let odds = Odds::new_american(american);
                b.iter(|| black_box(odds.to_decimal().unwrap()));
            },
        );
    }

    // Benchmark Decimal to American conversion
    for &decimal in &decimal_odds {
        group.bench_with_input(
            BenchmarkId::new("decimal_to_american", decimal.to_string()),
            &decimal,
            |b, &decimal| {
                let odds = Odds::new_decimal(decimal);
                b.iter(|| black_box(odds.to_american().unwrap()));
            },
        );
    }

    // Benchmark Fractional to Decimal conversion
    for &(num, den) in &fractional_odds {
        group.bench_with_input(
            BenchmarkId::new("fractional_to_decimal", format!("{}/{}", num, den)),
            &(num, den),
            |b, &(num, den)| {
                let odds = Odds::new_fractional(num, den);
                b.iter(|| black_box(odds.to_decimal().unwrap()));
            },
        );
    }

    // Benchmark Decimal to Fractional conversion
    for &decimal in &decimal_odds {
        group.bench_with_input(
            BenchmarkId::new("decimal_to_fractional", decimal.to_string()),
            &decimal,
            |b, &decimal| {
                let odds = Odds::new_decimal(decimal);
                b.iter(|| black_box(odds.to_fractional().unwrap()));
            },
        );
    }

    group.finish();
}

fn benchmark_probability_calculations(c: &mut Criterion) {
    let mut group = c.benchmark_group("probability");

    let test_odds = [
        Odds::new_american(100),
        Odds::new_american(-110),
        Odds::new_decimal(2.0),
        Odds::new_decimal(1.91),
        Odds::new_fractional(3, 2),
        Odds::new_fractional(1, 2),
    ];

    for (i, odds) in test_odds.iter().enumerate() {
        group.bench_with_input(
            BenchmarkId::new("implied_probability", i),
            odds,
            |b, odds| {
                b.iter(|| black_box(odds.implied_probability().unwrap()));
            },
        );
    }

    group.finish();
}

fn benchmark_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parsing");

    let test_strings = vec!["+150", "-200", "100", "2.50", "1.91", "3/2", "9/4", "1/2"];

    for string in &test_strings {
        group.bench_with_input(
            BenchmarkId::new("parse_from_string", *string),
            string,
            |b, &string| {
                b.iter(|| {
                    let parsed: Result<Odds, _> = black_box(string).parse();
                    black_box(parsed.unwrap())
                });
            },
        );
    }

    group.finish();
}

fn benchmark_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("formatting");

    let test_odds = [
        Odds::new_american(150),
        Odds::new_american(-200),
        Odds::new_decimal(2.5),
        Odds::new_fractional(3, 2),
    ];

    for (i, odds) in test_odds.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("format_to_string", i), odds, |b, odds| {
            b.iter(|| black_box(format!("{}", odds)));
        });
    }

    group.finish();
}

fn benchmark_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("validation");

    let test_odds = [
        Odds::new_american(150),
        Odds::new_american(0), // Invalid
        Odds::new_decimal(2.5),
        Odds::new_decimal(0.5), // Invalid
        Odds::new_fractional(3, 2),
        Odds::new_fractional(1, 0), // Invalid
    ];

    for (i, odds) in test_odds.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("validate", i), odds, |b, odds| {
            b.iter(|| black_box(odds.validate()));
        });
    }

    group.finish();
}

fn benchmark_roundtrip_conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("roundtrip");

    // Benchmark full roundtrip: American -> Decimal -> American
    group.bench_function("american_decimal_american", |b| {
        let odds = Odds::new_american(150);
        b.iter(|| {
            let decimal = black_box(odds.to_decimal().unwrap());
            let decimal_odds = Odds::new_decimal(decimal);
            black_box(decimal_odds.to_american().unwrap())
        });
    });

    // Benchmark full roundtrip: Decimal -> Fractional -> Decimal
    group.bench_function("decimal_fractional_decimal", |b| {
        let odds = Odds::new_decimal(2.5);
        b.iter(|| {
            let (num, den) = black_box(odds.to_fractional().unwrap());
            let fractional_odds = Odds::new_fractional(num, den);
            black_box(fractional_odds.to_decimal().unwrap())
        });
    });

    group.finish();
}

fn benchmark_construction(c: &mut Criterion) {
    let mut group = c.benchmark_group("construction");

    group.bench_function("new_american", |b| {
        b.iter(|| black_box(Odds::new_american(black_box(150))));
    });

    group.bench_function("new_decimal", |b| {
        b.iter(|| black_box(Odds::new_decimal(black_box(2.5))));
    });

    group.bench_function("new_fractional", |b| {
        b.iter(|| black_box(Odds::new_fractional(black_box(3), black_box(2))));
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_conversions,
    benchmark_probability_calculations,
    benchmark_parsing,
    benchmark_formatting,
    benchmark_validation,
    benchmark_roundtrip_conversions,
    benchmark_construction
);

criterion_main!(benches);
