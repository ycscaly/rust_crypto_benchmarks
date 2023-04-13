g.bench_function(
"benchmark Paillier (mod N^2 where N = P*Q and P, Q are 1024-bit primes) addition using crypto-bigint",
|bench| {
bench.iter_batched(
|| {
let P = crypto_primes::prime::<16>(1024);
let Q = crypto_primes::prime::<16>(1024);
let (N_low, N_high) = P.mul_wide(&Q);
let N_limbs: [Limb; 32] = concat_arrays!(N_low.to_limbs(), N_high.to_limbs());
let N = U2048::from(N_limbs);

let N2_low, N2_high = N.mul_wide(&N);
let N2_limbs: [Limb; 64] = concat_arrays!(N2_low.to_limbs(), N2_high.to_limbs());
let N2 = U4096::from(N2_limbs);

let params = DynResidueParams::new(&N2);
DynResidue::new(&N2, params)
},
|(x, y)| x.add_mod(&y, &ORDER),
BatchSize::SmallInput,
);
},
);