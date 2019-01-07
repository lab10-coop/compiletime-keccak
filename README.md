Procedural macro to calculate a keccak hash at compile-time and initialize a H256 for use at run-time.

Its main purpose at the time of writing is to avoid the overhead of repeatedly calculating a static keccak hash in environments where computation is expensive.
Examples of such an environments are pwasm and ewasm smart contracts.
