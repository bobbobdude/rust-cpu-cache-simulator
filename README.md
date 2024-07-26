# Rust Cache Simulator

This software aims to simulate the behaviour of a cache using tracefiles produced by Valgrind, and dependent upon whether the cache replacement policy is Direct Mapped (DMC), Fully Associative (FAC) or Set Associative (SAC), print out the correct number of Cache Misses, Hits and Evictions.

```
coursework-bobbobdude
├─ README.md
├─ coursework-details
│  ├─ CSM030-CompSys Coursework brief PDF.pdf
│  └─ The plan for the ArrayRepresentationOfCache (DMC).pdf
├─ sim
│  ├─ Cargo.lock
│  ├─ Cargo.toml
│  ├─ src
│  │  ├─ cache.rs
│  │  ├─ main.rs
│  │  └─ traces
│  │     ├─ custom.trace
│  │     ├─ custom2.trace
│  │     ├─ customWITHI.trace
│  │     ├─ ibm.trace
│  │     ├─ long.trace
│  │     ├─ yi.trace
│  │     └─ yi2.trace
├─ sim-ref
└─ traces
   ├─ custom.trace
   ├─ customWITHI.trace
   ├─ ibm.trace
   ├─ long.trace
   ├─ trans.trace
   ├─ yi.trace
   └─ yi2.trace

```
