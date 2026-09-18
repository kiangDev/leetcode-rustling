/*
Problem: 49. Group Anagrams
Link: https://leetcode.com/problems/group-anagrams/
Difficulty: Medium
Topic: Arrays & Hashing
Language: Rust

================================================================================
[Architectural Design & Engineering Trade-offs]
================================================================================

Approach 1: Sorted String as Key (Naive)
- Time: O(N * K log K) — Sorting each word of length K.
- Memory: Allocates brand new heap Strings for every key. High allocation churn.

Approach 2: Fixed Array [u8; 26] as Key (Optimal Systems Approach)
- Time: O(N * K) — Linear scan over all bytes. In terms of total input size (M = N*K), 
  this is strictly optimal O(M) and meets the theoretical lower bound.
- Memory: 26 bytes allocated on the stack for the key! 
  Fits in less than half a cache line (64 bytes).
- Zero-Copy Pipeline: Strings are moved (ownership transferred) directly into 
  the HashMap and collected via `into_values()` without heap re-allocation.

================================================================================
[Advanced Systems Nuances: Scaling K]
================================================================================
1. Type Widening (Data Bounds):
   - For K <= 255: `[u8; 26]` (26 bytes).
   - For K <= 65,535: `[u16; 26]` (52 bytes).
   - For K > 65,535: `[u32; 26]` (104 bytes).
   Even for massive strings (K = 1,000,000), the key remains a tiny 104-byte 
   stack allocation, whereas sorting would require millions of swapping ops.

2. Big Data Optimizations (When K is huge):
   - SIMD Vectorization: Processing 32 bytes per cycle via AVX2 instructions.
   - Commutative Hashing: Mapping characters to prime numbers and multiplying 
     modulo M (Order-independent hash), reducing key size to a single 8-byte u64.

3. Rust Idiomatic Entry API:
   - `map.entry(key).or_default().push(s)` performs a single hash lookup 
     for both search and insert, halving the CPU hashing overhead.
================================================================================
*/

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // Pre-allocate hash map capacity to reduce re-hashing
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::with_capacity(strs.len());

        for s in strs {
            // 26-byte stack allocation (< 1 cache line)
            let mut count = [0u8; 26];

            // Direct hardware AGU addressing
            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }

            // Single-lookup Entry API + Zero-copy ownership move
            map.entry(count).or_default().push(s);
        }

        // Consume map and extract values without data duplication
        map.into_values().collect()
    }
}