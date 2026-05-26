# SoK: ZK-Friendly Hash Functions over Prime Fields

This repository benchmarks plain permutation implementations of ZK-friendly hash functions.

## Prerequisites

- Rust 1.85+
- The project depends on `arkworks` (BN254, BLS12-381) and `Plonky3` (Goldilocks, Mersenne31, BabyBear, KoalaBear).

## Build & Run

```bash
RUSTFLAGS='-C target-cpu=native' cargo run --release > results.txt
```

## Benchmarked Primitives

**ZK-friendly hashes:**
Poseidon, Poseidon2, Griffin, Anemoi, Rescue-Prime, GMiMC-ERF, S-GMiMC, Neptune, ReinforcedConcrete, Monolith, Skyscraper, Tip4', Tip5, Arion, Polocolo

**Cryptographic Hash Functions:**
SHA-256, Keccak-f[1600], Blake2b, Blake3

**Fields:** BN254, BLS12-381 (~256-bit); Goldilocks (~64-bit);
Mersenne31, BabyBear, KoalaBear (~31-bit)

## Output

Each permutation is called 2^14 = 16,384 times on a fixed deterministic input.
Throughput is reported as average nanoseconds per call.
