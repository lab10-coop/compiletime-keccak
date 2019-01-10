A procedural Rust macro to calculate a keccak hash at compile-time.

Generates functions returning a H256 instance containing hash value baked at compile-time.

An example test comparing a run-time calculated hash to the hash calculated at compile-time:

```
keccak_derive::compiletime_keccak!(hashed_string);

#[test]
fn compare_compile_time_to_runtime_keccak() {
    let hash = keccak(b"hashed_string");
    assert_eq!(hashed_string(), hash);
}
```

Its main purpose at the time of writing is to avoid the overhead of repeatedly calculating a static keccak hash in environments where computation is expensive.
Examples of such an environments are pwasm and ewasm smart contracts.

