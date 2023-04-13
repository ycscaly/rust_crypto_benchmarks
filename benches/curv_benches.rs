#![feature(test)]

use curv::elliptic::curves::Secp256k1 as secp256k1;
use curv::elliptic::curves::Ed25519 as ed25519;
use curv::elliptic::curves::Secp256r1 as p256;
use curv::elliptic::curves::Ristretto as ristretto;
use curv::elliptic::curves::{Point, Scalar};

use criterion::{Criterion, BatchSize, criterion_main};

pub(crate) fn curv_benches() {
    let mut c = Criterion::default();
    let mut g = c.benchmark_group("curv benches");

    /* Scalar Addition */
    g.bench_function("benchmark scalar addition in secp256k1", |bench| {        
        bench.iter_batched(
            || ( Scalar::<secp256k1>::random(), Scalar::<secp256k1>::random()),
            |(x, y)| x + y,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar addition in ed25519", |bench| {        
        bench.iter_batched(
            || ( Scalar::<ed25519>::random(),  Scalar::<ed25519>::random()),
            |(x, y)| x + y,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar addition in p256", |bench| {        
        bench.iter_batched(
            || ( Scalar::<p256>::random(),  Scalar::<p256>::random()),
            |(x, y)| x + y,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar addition in ristretto", |bench| {        
        bench.iter_batched(
            || ( Scalar::<ristretto>::random(),  Scalar::<ristretto>::random()),
            |(x, y)| x + y,
            BatchSize::SmallInput,
        );
    });


    /* Scalar Multiplication */
    g.bench_function("benchmark scalar multiplication in secp256k1", |bench| {        
        bench.iter_batched(
            || ( Scalar::<secp256k1>::random(), Scalar::<secp256k1>::random()),
            |(x, y)| x * y,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar multiplication in ed25519", |bench| {        
        bench.iter_batched(
            || ( Scalar::<ed25519>::random(),  Scalar::<ed25519>::random()),
            |(x, y)| x * y,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar multiplication in p256", |bench| {        
        bench.iter_batched(
            || ( Scalar::<p256>::random(),  Scalar::<p256>::random()),
            |(x, y)| x * y,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar multiplication in ristretto", |bench| {        
        bench.iter_batched(
            || ( Scalar::<ristretto>::random(),  Scalar::<ristretto>::random()),
            |(x, y)| x * y,
            BatchSize::SmallInput,
        );
    });

    /* Scalar inversion */
    g.bench_function("benchmark scalar inversion in secp256k1", |bench| {        
        bench.iter_batched(
            || (Scalar::<secp256k1>::random()),
            |x| x.invert().unwrap(),
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar inversion in ed25519", |bench| {        
        bench.iter_batched(
            || (Scalar::<ed25519>::random()),
            |x| x.invert().unwrap(),
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar inversion in p256", |bench| {        
        bench.iter_batched(
            || (Scalar::<p256>::random()),
            |x| x.invert().unwrap(),
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark scalar inversion in ristretto", |bench| {        
        bench.iter_batched(
            || (Scalar::<ristretto>::random()),
            |x| x.invert().unwrap(),
            BatchSize::SmallInput,
        );
    });

    /* Exponentiation */
    g.bench_function("benchmark exponentiation in secp256k1", |bench| {        
        bench.iter_batched(
            || (Scalar::<secp256k1>::random(),  Point::<secp256k1>::generator()),
            |(x, g)| g * x,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark exponentiation in ed25519", |bench| {        
        bench.iter_batched(
            || (Scalar::<ed25519>::random(),  Point::<ed25519>::generator()),
            |(x, g)| g * x,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark exponentiation in p256", |bench| {        
        bench.iter_batched(
            || (Scalar::<p256>::random(),  Point::<p256>::generator()),
            |(x, g)| g * x,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark exponentiation in ristretto", |bench| {        
        bench.iter_batched(
            || (Scalar::<ristretto>::random(),  Point::<ristretto>::generator()),
            |(x, g)| g * x,
            BatchSize::SmallInput,
        );
    });

    /* Point Addition */
    g.bench_function("benchmark point addition in secp256k1", |bench| {        
        bench.iter_batched(
            || ( Point::<secp256k1>::generator() * Scalar::<secp256k1>::random(),  Point::<secp256k1>::generator() * Scalar::<secp256k1>::random()),
            |(p1, p2)| p1 + p2,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark point addition in ed25519", |bench| {        
        bench.iter_batched(
            || ( Point::<ed25519>::generator() * Scalar::<ed25519>::random(),  Point::<ed25519>::generator() * Scalar::<ed25519>::random()),
            |(p1, p2)| p1 + p2,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark point addition in p256", |bench| {        
        bench.iter_batched(
            || ( Point::<p256>::generator() * Scalar::<p256>::random(),  Point::<p256>::generator() * Scalar::<p256>::random()),
            |(p1, p2)| p1 + p2,
            BatchSize::SmallInput,
        );
    });

    g.bench_function("benchmark point addition in ristretto", |bench| {        
        bench.iter_batched(
            || ( Point::<ristretto>::generator() * Scalar::<ristretto>::random(),  Point::<ristretto>::generator() * Scalar::<ristretto>::random()),
            |(p1, p2)| p1 + p2,
            BatchSize::SmallInput,
        );
    });

    g.finish();
}

criterion_main!(curv_benches);
