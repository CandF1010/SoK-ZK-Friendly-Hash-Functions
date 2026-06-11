# Hash Function Implementations in Dusk Network Plonk
This crate implements various Zero-Knowledge-friendly hash functions in [Dusk Network Plonk](https://github.com/dusk-network/plonk) to compare their performance operating on `Scalar`, and for a zero-knowledge circuit proof generation and verification.
The following hash functions are implemented:
- Anemoi
- Arion
- GMiMC
- GMiMC2
- Grendel
- Griffin
- Poseidon
- Poseidon2
- Rescue
- RescuePrime

The state size of all primitives is set to `WIDTH = 4`.

The implementations are based on [Dusk Network's Poseidon reference implementation](https://github.com/dusk-network/Poseidon252).
In particular, the Poseidon implementation is forked from commit `a4447b9c4a3b45dfb8128f7eaf12e2dd02dd9c29`.

## Benchmarks
There are benchmarks for hashing, encrypting and decrypting in their native form, operating on `Scalar`, and for a zero-knowledge circuit proof generation and verification.

To run all benchmarks either run
```shell
cargo bench --features=zk,encryption,bls-backend-blst
```
or
```shell
cargo bench --features=zk,encryption,bls-backend-dusk
```
in the repository.

To run hash benchmarks only run
```shell
cargo bench --features=zk,bls-backend-blst
```
or
```shell
cargo bench --features=zk,bls-backend-dusk
```
in the repository.

## Tests
To run all tests either run
```shell
cargo test --features=zk,encryption,bls-backend-blst
```
or
```shell
cargo test --features=zk,encryption,bls-backend-dusk
```
in the repository.
