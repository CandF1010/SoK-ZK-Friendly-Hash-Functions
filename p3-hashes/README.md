# Hash Function Implementations in Plonky3
This crate implements various Zero-Knowledge-friendly hash functions in [Plonky3](https://github.com/Plonky3/Plonky3) to compare their performance operating in plain and in AIR.
The following hash functions are implemented:
- GMiMC
- GMiMC2
- XHash

The XHash implementation is forked from https://github.com/Plonky3/Plonky3/tree/robin/xhash commit `5428518c754e26674aa43712474b401253333673`.

## Benchmarks
To run all benchmarks simply run
```shell
cargo bench
```
