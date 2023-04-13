#![feature(test)]

use crypto_bigint::{impl_modulus, Limb, NonZero, RandomMod, Word, U2048, U256, U4096};
use elliptic_curve::rand_core;

use criterion::{criterion_main, BatchSize, Criterion};

use crypto_bigint::const_residue;
use crypto_bigint::modular::constant_mod::{Residue, ResidueParams};
use crypto_bigint::Encoding;

/// Order of the secp256k1 elliptic curve in hexadecimal.
const ORDER_HEX: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";

/// Order of the secp256k1 elliptic curve.
const ORDER: U256 = U256::from_be_hex(ORDER_HEX);

impl_modulus!(K256Modulus, U256, ORDER_HEX);

pub(crate) fn crypto_bigint_benches() {
    let mut c = Criterion::default();
    let mut g = c.benchmark_group("crypto-bigint benches");

    g.bench_function(
        "benchmark scalar addition in secp256k1 using crypto-bigint",
        |bench| {
            bench.iter_batched(
                || {
                    (
                        U256::random_mod(&mut rand_core::OsRng, &NonZero::new(ORDER).unwrap()),
                        U256::random_mod(&mut rand_core::OsRng, &NonZero::new(ORDER).unwrap()),
                    )
                },
                |(x, y)| x.add_mod(&y, &ORDER),
                BatchSize::SmallInput,
            );
        },
    );

    g.bench_function(
        "benchmark scalar addition in secp256k1 using crypto-bigint residue",
        |bench| {
            bench.iter_batched(
                || {
                    let x: U256 =
                        U256::random_mod(&mut rand_core::OsRng, &NonZero::new(ORDER).unwrap());
                    let y: U256 =
                        U256::random_mod(&mut rand_core::OsRng, &NonZero::new(ORDER).unwrap());
                    (
                        const_residue!(x, K256Modulus),
                        const_residue!(y, K256Modulus),
                    )
                },
                |(x, y)| x + y,
                BatchSize::SmallInput,
            );
        },
    );

    g.bench_function(
        "benchmark scalar multiplication in secp256k1 using crypto-bigint residue",
        |bench| {
            bench.iter_batched(
                || {
                    let x: U256 =
                        U256::random_mod(&mut rand_core::OsRng, &NonZero::new(ORDER).unwrap());
                    let y: U256 =
                        U256::random_mod(&mut rand_core::OsRng, &NonZero::new(ORDER).unwrap());
                    (
                        const_residue!(x, K256Modulus),
                        const_residue!(y, K256Modulus),
                    )
                },
                |(x, y)| x * y,
                BatchSize::SmallInput,
            );
        },
    );

    g.bench_function(
        "benchmark scalar inversion in secp256k1 using crypto-bigint residue",
        |bench| {
            bench.iter_batched(
                || {
                    let x: U256 =
                        U256::random_mod(&mut rand_core::OsRng, &NonZero::new(ORDER).unwrap());
                    const_residue!(x, K256Modulus)
                },
                |x| x.invert(),
                BatchSize::SmallInput,
            );
        },
    );

    g.finish();
}

criterion_main!(crypto_bigint_benches);
