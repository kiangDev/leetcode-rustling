/*
Problem: 217. Contains Duplicate
Link: https://leetcode.com/problems/contains-duplicate/
Difficulty: Easy
Topic: Arrays & Hashing
Language: Rust

================================================================================
[Engineering Trade-offs & Approaches]
================================================================================

Approach 1: HashSet (Optimal Time: O(N) Time, O(N) Space)
- Best for: Latency-critical systems with sufficient RAM.
- Mechanism: Single-pass hashing using `!seen.insert(num)` for early exit.
- Memory Nuance: `with_capacity(nums.len())` prevents repeated heap re-allocations 
  in worst-case scenarios, while `new()` is better if duplicates appear early.

Approach 2: In-place Sorting + Windows (Optimal Space: O(N log N) Time, O(1) Space)
- Best for: Memory-constrained environments (Embedded, IoT, low-RAM containers).
- Mechanism: 
  1. Re-bind `mut nums = nums` (Variable Shadowing) to mutate in-place.
  2. `nums.sort_unstable()` uses pdqsort with zero extra heap allocation.
  3. `nums.windows(2).any(|w| w[0] == w[1])` creates a sliding window iterator 
     of size 2 with short-circuit evaluation (lazy evaluation stops at first match).

================================================================================
[Complexity Comparison]
================================================================================
| Approach                   | Time       | Space  | Allocations | Cache Friendly |
|----------------------------|------------|--------|-------------|----------------|
| 1. HashSet                 | O(N)       | O(N)   | Heap        | Low            |
| 2. Sort + Windows (pdqsort)| O(N log N) | O(1)   | Zero        | High           |
================================================================================
[Acknowledgments & Inspiration]
- Approach 2 was inspired by the idiomatic functional Rust solutions shared 
  by the LeetCode community 
================================================================================
*/

use std::collections::HashSet;

impl Solution {
    // ==========================================
    // Approach 1: HashSet (O(N) Time, O(N) Space)
    // ==========================================
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen = HashSet::with_capacity(nums.len());

        for num in nums {
            if !seen.insert(num) {
                return true; // Duplicate found (Early Exit)
            }
        }

        return false;
    }

    // 
    // =========================================================
    // Approach 2: In-place Sort + Windows (O(N log N) Time, O(1) Space)
    // =========================================================
    /*
    pub fn contains_duplicate_sorting(nums: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort_unstable();
        nums.windows(2).any(|w| w[0] == w[1])
    }
    */
}