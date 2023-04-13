Benchmarks of cryptographic code in Rust
To run: cargo bench
--------
Latest results (Macbook M1 Pro 14'):
```
➜  rust-crypto-benchmarks git:(master) ✗ cargo bench
   Compiling bench_curves v0.1.0 (/Users/jcscaly/projects/rust-crypto-benchmarks)
warning: unused imports: `Limb`, `U2048`, `U4096`, `Word`
 --> benches/crypto_bigint_benches.rs:3:35
  |
3 | use crypto_bigint::{impl_modulus, Limb, NonZero, RandomMod, Word, U2048, U256, U4096};
  |                                   ^^^^                      ^^^^  ^^^^^        ^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `Residue`
 --> benches/crypto_bigint_benches.rs:9:44
  |
9 | use crypto_bigint::modular::constant_mod::{Residue, ResidueParams};
  |                                            ^^^^^^^

warning: unused import: `crypto_bigint::Encoding`
  --> benches/crypto_bigint_benches.rs:10:5
   |
10 | use crypto_bigint::Encoding;
   |     ^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `black_box`
 --> benches/secp256k1_benches.rs:1:17
  |
1 | use criterion::{black_box, criterion_main, BatchSize, Criterion};
  |                 ^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `crypto_bigint::Limb`
 --> benches/secp256k1_benches.rs:2:5
  |
2 | use crypto_bigint::Limb;
  |     ^^^^^^^^^^^^^^^^^^^

warning: unused import: `k256::Secp256k1`
 --> benches/secp256k1_benches.rs:7:5
  |
7 | use k256::Secp256k1;
  |     ^^^^^^^^^^^^^^^

warning: `bench_curves` (bench "secp256k1_benches") generated 3 warnings (run `cargo fix --bench "secp256k1_benches"` to apply 3 suggestions)
warning: `bench_curves` (bench "crypto_bigint_benches") generated 3 warnings (run `cargo fix --bench "crypto_bigint_benches"` to apply 3 suggestions)
    Finished bench [optimized] target(s) in 2.28s
     Running unittests src/lib.rs (target/release/deps/bench_curves-a46e7a5cebaeb616)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/crypto_bigint_benches.rs (target/release/deps/crypto_bigint_benches-b50b1c183a404906)
Gnuplot not found, using plotters backend
Benchmarking crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint
Benchmarking crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint: Warming up for 3.0000 s
Benchmarking crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint: Collecting 100 samples in estimated 5.0036 s (2.6M iterations)
Benchmarking crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint: Analyzing
crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint
                        time:   [3.8404 ns 4.4116 ns 5.2732 ns]
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe
Benchmarking crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint residue
Benchmarking crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint residue: Warming up for 3.0000 s
Benchmarking crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint residue: Collecting 100 samples in estimated 5.0022 s (2.5M iterations)
Benchmarking crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint residue: Analyzing
crypto-bigint benches/benchmark scalar addition in secp256k1 using crypto-bigint residue
                        time:   [3.8627 ns 4.1233 ns 4.5194 ns]
Found 16 outliers among 100 measurements (16.00%)
  5 (5.00%) high mild
  11 (11.00%) high severe
Benchmarking crypto-bigint benches/benchmark scalar multiplication in secp256k1 using crypto-bigint residue
Benchmarking crypto-bigint benches/benchmark scalar multiplication in secp256k1 using crypto-bigint residue: Warming up for 3.0000 s
Benchmarking crypto-bigint benches/benchmark scalar multiplication in secp256k1 using crypto-bigint residue: Collecting 100 samples in estimated 5.0028 s (2.4M iterations)
Benchmarking crypto-bigint benches/benchmark scalar multiplication in secp256k1 using crypto-bigint residue: Analyzing
crypto-bigint benches/benchmark scalar multiplication in secp256k1 using crypto-bigint residue
                        time:   [27.084 ns 28.693 ns 30.497 ns]
Found 19 outliers among 100 measurements (19.00%)
  1 (1.00%) high mild
  18 (18.00%) high severe
Benchmarking crypto-bigint benches/benchmark scalar inversion in secp256k1 using crypto-bigint residue
Benchmarking crypto-bigint benches/benchmark scalar inversion in secp256k1 using crypto-bigint residue: Warming up for 3.0000 s
Benchmarking crypto-bigint benches/benchmark scalar inversion in secp256k1 using crypto-bigint residue: Collecting 100 samples in estimated 5.0057 s (328k iterations)
Benchmarking crypto-bigint benches/benchmark scalar inversion in secp256k1 using crypto-bigint residue: Analyzing
crypto-bigint benches/benchmark scalar inversion in secp256k1 using crypto-bigint residue
                        time:   [13.968 µs 14.008 µs 14.052 µs]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

     Running benches/curv_benches.rs (target/release/deps/curv_benches-aa48987e0e891083)
Gnuplot not found, using plotters backend
Benchmarking curv benches/benchmark scalar addition in secp256k1
Benchmarking curv benches/benchmark scalar addition in secp256k1: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar addition in secp256k1: Collecting 100 samples in estimated 5.0001 s (41M iterations)
Benchmarking curv benches/benchmark scalar addition in secp256k1: Analyzing
curv benches/benchmark scalar addition in secp256k1
                        time:   [61.184 ns 61.370 ns 61.564 ns]
                        change: [-2.6531% -2.0343% -1.4097%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Benchmarking curv benches/benchmark scalar addition in ed25519
Benchmarking curv benches/benchmark scalar addition in ed25519: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar addition in ed25519: Collecting 100 samples in estimated 5.0010 s (1.6M iterations)
Benchmarking curv benches/benchmark scalar addition in ed25519: Analyzing
curv benches/benchmark scalar addition in ed25519
                        time:   [541.45 ns 543.08 ns 544.72 ns]
                        change: [-3.6512% -2.8452% -2.0692%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
Benchmarking curv benches/benchmark scalar addition in p256
Benchmarking curv benches/benchmark scalar addition in p256: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar addition in p256: Collecting 100 samples in estimated 5.0005 s (30M iterations)
Benchmarking curv benches/benchmark scalar addition in p256: Analyzing
curv benches/benchmark scalar addition in p256
                        time:   [8.0969 ns 8.3359 ns 8.5633 ns]
                        change: [-17.863% -2.3295% +16.305%] (p = 0.83 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Benchmarking curv benches/benchmark scalar addition in ristretto
Benchmarking curv benches/benchmark scalar addition in ristretto: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar addition in ristretto: Collecting 100 samples in estimated 5.0018 s (8.6M iterations)
Benchmarking curv benches/benchmark scalar addition in ristretto: Analyzing
curv benches/benchmark scalar addition in ristretto
                        time:   [70.258 ns 70.564 ns 70.931 ns]
                        change: [-2.0245% -0.3438% +1.9900%] (p = 0.81 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
Benchmarking curv benches/benchmark scalar multiplication in secp256k1
Benchmarking curv benches/benchmark scalar multiplication in secp256k1: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar multiplication in secp256k1: Collecting 100 samples in estimated 5.0000 s (34M iterations)
Benchmarking curv benches/benchmark scalar multiplication in secp256k1: Analyzing
curv benches/benchmark scalar multiplication in secp256k1
                        time:   [87.045 ns 87.290 ns 87.581 ns]
                        change: [-2.3296% -1.2513% -0.0766%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
Benchmarking curv benches/benchmark scalar multiplication in ed25519
Benchmarking curv benches/benchmark scalar multiplication in ed25519: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar multiplication in ed25519: Collecting 100 samples in estimated 5.0126 s (1.6M iterations)
Benchmarking curv benches/benchmark scalar multiplication in ed25519: Analyzing
curv benches/benchmark scalar multiplication in ed25519
                        time:   [727.09 ns 728.90 ns 730.79 ns]
                        change: [-1.7557% -1.1417% -0.4288%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
Benchmarking curv benches/benchmark scalar multiplication in p256
Benchmarking curv benches/benchmark scalar multiplication in p256: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar multiplication in p256: Collecting 100 samples in estimated 5.0009 s (25M iterations)
Benchmarking curv benches/benchmark scalar multiplication in p256: Analyzing
curv benches/benchmark scalar multiplication in p256
                        time:   [42.106 ns 42.355 ns 42.607 ns]
                        change: [-10.729% -6.7631% -4.1174%] (p = 0.00 < 0.05)
                        Performance has improved.
Benchmarking curv benches/benchmark scalar multiplication in ristretto
Benchmarking curv benches/benchmark scalar multiplication in ristretto: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar multiplication in ristretto: Collecting 100 samples in estimated 5.0027 s (8.2M iterations)
Benchmarking curv benches/benchmark scalar multiplication in ristretto: Analyzing
curv benches/benchmark scalar multiplication in ristretto
                        time:   [97.779 ns 98.148 ns 98.561 ns]
                        change: [-3.9266% -2.0189% -0.7165%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking curv benches/benchmark scalar inversion in secp256k1
Benchmarking curv benches/benchmark scalar inversion in secp256k1: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar inversion in secp256k1: Collecting 100 samples in estimated 5.1781 s (66k iterations)
Benchmarking curv benches/benchmark scalar inversion in secp256k1: Analyzing
curv benches/benchmark scalar inversion in secp256k1
                        time:   [78.771 µs 78.868 µs 78.980 µs]
                        change: [-1.3372% -0.8270% -0.3981%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  5 (5.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe
Benchmarking curv benches/benchmark scalar inversion in ed25519
Benchmarking curv benches/benchmark scalar inversion in ed25519: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar inversion in ed25519: Collecting 100 samples in estimated 5.2138 s (66k iterations)
Benchmarking curv benches/benchmark scalar inversion in ed25519: Analyzing
curv benches/benchmark scalar inversion in ed25519
                        time:   [78.995 µs 80.027 µs 81.556 µs]
                        change: [+1.0981% +2.0662% +3.4457%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
Benchmarking curv benches/benchmark scalar inversion in p256
Benchmarking curv benches/benchmark scalar inversion in p256: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar inversion in p256: Collecting 100 samples in estimated 5.0787 s (237k iterations)
Benchmarking curv benches/benchmark scalar inversion in p256: Analyzing
curv benches/benchmark scalar inversion in p256
                        time:   [21.233 µs 21.291 µs 21.356 µs]
                        change: [-0.5072% -0.1939% +0.1419%] (p = 0.24 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
Benchmarking curv benches/benchmark scalar inversion in ristretto
Benchmarking curv benches/benchmark scalar inversion in ristretto: Warming up for 3.0000 s
Benchmarking curv benches/benchmark scalar inversion in ristretto: Collecting 100 samples in estimated 5.0250 s (490k iterations)
Benchmarking curv benches/benchmark scalar inversion in ristretto: Analyzing
curv benches/benchmark scalar inversion in ristretto
                        time:   [9.9887 µs 10.002 µs 10.018 µs]
                        change: [+0.3404% +0.5762% +0.8174%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
Benchmarking curv benches/benchmark exponentiation in secp256k1
Benchmarking curv benches/benchmark exponentiation in secp256k1: Warming up for 3.0000 s
Benchmarking curv benches/benchmark exponentiation in secp256k1: Collecting 100 samples in estimated 5.0743 s (293k iterations)
Benchmarking curv benches/benchmark exponentiation in secp256k1: Analyzing
curv benches/benchmark exponentiation in secp256k1
                        time:   [17.103 µs 17.120 µs 17.137 µs]
                        change: [-1.4573% -1.1092% -0.7857%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
Benchmarking curv benches/benchmark exponentiation in ed25519
Benchmarking curv benches/benchmark exponentiation in ed25519: Warming up for 3.0000 s
Benchmarking curv benches/benchmark exponentiation in ed25519: Collecting 100 samples in estimated 5.9436 s (30k iterations)
Benchmarking curv benches/benchmark exponentiation in ed25519: Analyzing
curv benches/benchmark exponentiation in ed25519
                        time:   [195.02 µs 195.51 µs 196.02 µs]
                        change: [-0.5925% -0.3421% -0.1046%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
Benchmarking curv benches/benchmark exponentiation in p256
Benchmarking curv benches/benchmark exponentiation in p256: Warming up for 3.0000 s
Benchmarking curv benches/benchmark exponentiation in p256: Collecting 100 samples in estimated 5.2546 s (50k iterations)
Benchmarking curv benches/benchmark exponentiation in p256: Analyzing
curv benches/benchmark exponentiation in p256
                        time:   [103.78 µs 103.96 µs 104.14 µs]
                        change: [-1.4060% -0.8824% -0.4237%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
Benchmarking curv benches/benchmark exponentiation in ristretto
Benchmarking curv benches/benchmark exponentiation in ristretto: Warming up for 3.0000 s
Benchmarking curv benches/benchmark exponentiation in ristretto: Collecting 100 samples in estimated 5.0985 s (162k iterations)
Benchmarking curv benches/benchmark exponentiation in ristretto: Analyzing
curv benches/benchmark exponentiation in ristretto
                        time:   [31.425 µs 31.605 µs 31.851 µs]
                        change: [-0.7663% -0.1190% +0.4256%] (p = 0.72 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Benchmarking curv benches/benchmark point addition in secp256k1
Benchmarking curv benches/benchmark point addition in secp256k1: Warming up for 3.0000 s
Benchmarking curv benches/benchmark point addition in secp256k1: Collecting 100 samples in estimated 5.0599 s (136k iterations)
Benchmarking curv benches/benchmark point addition in secp256k1: Analyzing
curv benches/benchmark point addition in secp256k1
                        time:   [2.3967 µs 2.4164 µs 2.4421 µs]
                        change: [-0.9339% +0.0558% +0.9479%] (p = 0.92 > 0.05)
                        No change in performance detected.
Found 17 outliers among 100 measurements (17.00%)
  10 (10.00%) high mild
  7 (7.00%) high severe
Benchmarking curv benches/benchmark point addition in ed25519
Benchmarking curv benches/benchmark point addition in ed25519: Warming up for 3.0000 s
Benchmarking curv benches/benchmark point addition in ed25519: Collecting 100 samples in estimated 6.1490 s (15k iterations)
Benchmarking curv benches/benchmark point addition in ed25519: Analyzing
curv benches/benchmark point addition in ed25519
                        time:   [13.598 µs 13.617 µs 13.641 µs]
                        change: [-1.3154% -0.9397% -0.5827%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
Benchmarking curv benches/benchmark point addition in p256
Benchmarking curv benches/benchmark point addition in p256: Warming up for 3.0000 s
Benchmarking curv benches/benchmark point addition in p256: Collecting 100 samples in estimated 5.3944 s (25k iterations)
Benchmarking curv benches/benchmark point addition in p256: Analyzing
curv benches/benchmark point addition in p256
                        time:   [6.1022 µs 6.1107 µs 6.1194 µs]
                        change: [-0.7496% -0.3432% -0.0176%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
Benchmarking curv benches/benchmark point addition in ristretto
Benchmarking curv benches/benchmark point addition in ristretto: Warming up for 3.0000 s
Benchmarking curv benches/benchmark point addition in ristretto: Collecting 100 samples in estimated 5.1031 s (81k iterations)
Benchmarking curv benches/benchmark point addition in ristretto: Analyzing
curv benches/benchmark point addition in ristretto
                        time:   [132.62 ns 142.08 ns 157.01 ns]
                        change: [-10.883% -1.5903% +8.3949%] (p = 0.77 > 0.05)
                        No change in performance detected.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

     Running benches/secp256k1_benches.rs (target/release/deps/secp256k1_benches-556986ca88fd3e5e)
Gnuplot not found, using plotters backend
Benchmarking secp256k1 benches/benchmark scalar addition in secp256k1 using fiat-crypto
Benchmarking secp256k1 benches/benchmark scalar addition in secp256k1 using fiat-crypto: Warming up for 3.0000 s
Benchmarking secp256k1 benches/benchmark scalar addition in secp256k1 using fiat-crypto: Collecting 100 samples in estimated 5.0012 s (9.1M iterations)
Benchmarking secp256k1 benches/benchmark scalar addition in secp256k1 using fiat-crypto: Analyzing
secp256k1 benches/benchmark scalar addition in secp256k1 using fiat-crypto
                        time:   [3.6132 ns 3.7160 ns 3.8718 ns]
                        change: [-2.1150% +14.299% +43.171%] (p = 0.38 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  6 (6.00%) high mild
  8 (8.00%) high severe
Benchmarking secp256k1 benches/benchmark scalar addition in secp256k1 using k256
Benchmarking secp256k1 benches/benchmark scalar addition in secp256k1 using k256: Warming up for 3.0000 s
Benchmarking secp256k1 benches/benchmark scalar addition in secp256k1 using k256: Collecting 100 samples in estimated 5.0014 s (9.0M iterations)
Benchmarking secp256k1 benches/benchmark scalar addition in secp256k1 using k256: Analyzing
secp256k1 benches/benchmark scalar addition in secp256k1 using k256
                        time:   [5.2289 ns 5.3555 ns 5.5127 ns]
                        change: [-10.072% +25.594% +86.068%] (p = 0.59 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) high mild
  7 (7.00%) high severe
Benchmarking secp256k1 benches/benchmark scalar multiplication in secp256k1 using fiat-crypto
Benchmarking secp256k1 benches/benchmark scalar multiplication in secp256k1 using fiat-crypto: Warming up for 3.0000 s
Benchmarking secp256k1 benches/benchmark scalar multiplication in secp256k1 using fiat-crypto: Collecting 100 samples in estimated 5.0005 s (8.8M iterations)
Benchmarking secp256k1 benches/benchmark scalar multiplication in secp256k1 using fiat-crypto: Analyzing
secp256k1 benches/benchmark scalar multiplication in secp256k1 using fiat-crypto
                        time:   [22.409 ns 22.674 ns 23.001 ns]
                        change: [-15.844% -1.3229% +12.795%] (p = 0.86 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe
Benchmarking secp256k1 benches/benchmark scalar multiplication in secp256k1 using k256
Benchmarking secp256k1 benches/benchmark scalar multiplication in secp256k1 using k256: Warming up for 3.0000 s
Benchmarking secp256k1 benches/benchmark scalar multiplication in secp256k1 using k256: Collecting 100 samples in estimated 5.0004 s (8.8M iterations)
Benchmarking secp256k1 benches/benchmark scalar multiplication in secp256k1 using k256: Analyzing
secp256k1 benches/benchmark scalar multiplication in secp256k1 using k256
                        time:   [28.934 ns 29.153 ns 29.429 ns]
                        change: [-0.1389% +3.7946% +10.013%] (p = 0.21 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

➜  rust-crypto-benchmarks git:(master) ✗
```