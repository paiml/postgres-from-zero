//! Criterion bench for the contract assertion functions. The DB-coupled
//! report path is intentionally excluded — micro-benching against
//! Postgres adds noise from the network round-trip without telling us
//! anything about the assertion overhead we actually control.

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// `assert_topn_invariants` and the row-shape asserts are private; benching
// the public `top_*` API would require a live database, so instead we
// exercise the assertion overhead via a synthetic shape that mirrors what
// the asserts look at: a sorted, fixed-shape Vec with a numeric metric.

fn synthetic_rows(n: usize) -> Vec<(i32, String, i64)> {
    (0..n)
        .map(|i| (i as i32 + 1, format!("NAME {i}"), (n - i) as i64))
        .collect()
}

fn check_invariants(rows: &[(i32, String, i64)]) {
    assert!(!rows.is_empty());
    assert!(rows[0].2 >= 1);
    for w in rows.windows(2) {
        assert!(w[0].2 >= w[1].2);
    }
    for r in rows {
        assert!(r.0 > 0);
        assert!(!r.1.is_empty());
    }
}

fn bench_invariants(c: &mut Criterion) {
    let rows_10 = synthetic_rows(10);
    let rows_100 = synthetic_rows(100);
    let rows_1000 = synthetic_rows(1000);

    c.bench_function("invariants_10", |b| {
        b.iter(|| check_invariants(black_box(&rows_10)))
    });
    c.bench_function("invariants_100", |b| {
        b.iter(|| check_invariants(black_box(&rows_100)))
    });
    c.bench_function("invariants_1000", |b| {
        b.iter(|| check_invariants(black_box(&rows_1000)))
    });
}

criterion_group!(benches, bench_invariants);
criterion_main!(benches);
