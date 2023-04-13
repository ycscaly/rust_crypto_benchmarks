#![feature(test)]

#[cfg(test)]
mod tests {
    
    // #[bench]
    // fn bench_exponentiation_secp256k1(b: &mut Bencher) {
    //     b.iter(|| {
    //         for i in 0..1000 {
    //             let exponent = secp256k1::Scalar::random();
    //             test::black_box(Point::<secp256k1>::generator() * exponent);
    //         }
    //     });
    // }
    // #[bench]
    // fn bench_exponentiation_ed25519(b: &mut Bencher) {
    //     b.iter(|| {
    //         for i in 0..1000 {
    //             let exponent = ed25519::Scalar::random();
    //             test::black_box(Point::<ed25519>::generator() * exponent);
    //         }
    //     });
    // }
    // #[bench]
    // fn bench_exponentiation_ristretto(b: &mut Bencher) {
    //     b.iter(|| {
    //         for i in 0..1000 {
    //             let exponent = ristretto::Scalar::random();
    //             test::black_box(Point::<ristretto>::generator() * exponent);
    //         }
    //     });
    // }
    // #[bench]
    // fn bench_exponentiation_p256(b: &mut Bencher) {
    //     b.iter(|| {
    //         for i in 0..1000 {
    //             let exponent = p256::Scalar::random();
    //             test::black_box(Point::<p256>::generator() * exponent);
    //         }
    //     });
    // }
}