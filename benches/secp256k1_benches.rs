use criterion::{criterion_main, BatchSize, Criterion};
use elliptic_curve::rand_core;
use elliptic_curve::Field;
use fiat_crypto::secp256k1_montgomery_64::*;
use k256::Scalar;
use k256::U256;

pub(crate) fn secp256k1_benches() {
    let mut c = Criterion::default();
    let mut g = c.benchmark_group("secp256k1 benches");

    g.bench_function(
        "benchmark scalar addition in secp256k1 using fiat-crypto",
        |bench| {
            bench.iter_batched(
                || {
                    let x: fiat_secp256k1_montgomery_montgomery_domain_field_element =
                        U256::from(Scalar::random(&mut rand_core::OsRng)).to_words();

                    let y: fiat_secp256k1_montgomery_montgomery_domain_field_element =
                        U256::from(Scalar::random(&mut rand_core::OsRng)).to_words();

                    (x, y)
                },
                |(x, y)| {
                    let mut out: fiat_secp256k1_montgomery_montgomery_domain_field_element =
                        [0u64; 4];
                    fiat_secp256k1_montgomery_add(&mut out, &x, &y);
                    out
                },
                BatchSize::SmallInput,
            );
        },
    );

    g.bench_function(
        "benchmark scalar addition in secp256k1 using k256",
        |bench| {
            bench.iter_batched(
                || {
                    (
                        Scalar::random(&mut rand_core::OsRng),
                        Scalar::random(&mut rand_core::OsRng),
                    )
                },
                |(x, y)| x + y,
                BatchSize::SmallInput,
            );
        },
    );

    g.bench_function(
        "benchmark scalar multiplication in secp256k1 using fiat-crypto",
        |bench| {
            bench.iter_batched(
                || {
                    let x: fiat_secp256k1_montgomery_montgomery_domain_field_element =
                        U256::from(Scalar::random(&mut rand_core::OsRng)).to_words();

                    let y: fiat_secp256k1_montgomery_montgomery_domain_field_element =
                        U256::from(Scalar::random(&mut rand_core::OsRng)).to_words();

                    (x, y)
                },
                |(x, y)| {
                    let mut out: fiat_secp256k1_montgomery_montgomery_domain_field_element =
                        [0u64; 4];
                    fiat_secp256k1_montgomery_mul(&mut out, &x, &y);
                    out
                },
                BatchSize::SmallInput,
            );
        },
    );

    g.bench_function(
        "benchmark scalar multiplication in secp256k1 using k256",
        |bench| {
            bench.iter_batched(
                || {
                    (
                        Scalar::random(&mut rand_core::OsRng),
                        Scalar::random(&mut rand_core::OsRng),
                    )
                },
                |(x, y)| x * y,
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_main!(secp256k1_benches);
