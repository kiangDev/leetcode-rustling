/*
Problem: 1. Two Sum
Link: https://leetcode.com/problems/two-sum/
Difficulty: Easy
Topic: Arrays & Hashing
Language: Rust

[Intuition & Approach]
- Brute force using nested loops takes O(N^2) time complexity, which is not optimal.
- By calculating the complement (target - num) and using a HashMap, we reduce the lookup time to O(1) average, achieving an overall O(N) time complexity.
- Loop through elements one by one: if the complement exists in the HashMap, return both indices immediately. Otherwise, insert the current number and its index into the HashMap for future lookups.

[Rust Key Takeaways]
1. `use std::collections::HashMap;` to import HashMap from the standard library.
2. `let mut map = HashMap::new();` creates a mutable HashMap instance.
3. `nums.iter().enumerate()` iterates with (index, value) by borrowing, avoiding moving ownership. Using `&num` unpacks the reference directly into an `i32`.
4. `map.get(&key)` looks up by reference and returns an `Option<&Value>` (`Some(&index)` or `None`).
5. In Rust, array indices are of type `usize`, so we cast them to `i32` with `as i32` to match the function's return signature.

[Complexity Analysis]
- Time Complexity: O(N) — One pass through the array.
- Space Complexity: O(N) — In the worst case, the HashMap stores up to N elements.
*/

use std::collections::HashMap; 

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            match map.get(&complement) {
                Some(&prev_index) => {
                    return vec![prev_index as i32, i as i32];
                }
                None => {
                    map.insert(num, i);
                }
            }
        }
        vec![]
    }
}

/*
================================================================================
[Production-Ready Architecture: The Hybrid Two-Sum]
================================================================================
In real-world production systems, data sizes vary widely. A Senior Systems Engineer
often implements a Hybrid algorithm that chooses the optimal path based on N:

1. Fast Path (N <= 32):
   - Small arrays fit entirely within a single 64-byte L1 Cache Line.
   - Brute Force O(N^2) nested loop runs in ~2-3 nanoseconds on CPU registers/L1 cache.
   - Completely bypasses Heap allocations, SipHash computation, and Map overhead.

2. General Path (N > 32):
   - Switches to HashMap O(N) to prevent quadratic time explosion (O(N^2) scaling).
   - Pre-allocates map capacity to eliminate re-hashing overhead.
================================================================================
*/

/*
impl Solution {
    pub fn two_sum_production(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let n = nums.len();

        // Fast Path: N <= 32 (L1 Cache Resident, Zero Heap Allocations)
        if n <= 32 {
            for i in 0..n {
                for j in (i + 1)..n {
                    if nums[i] + nums[j] == target {
                        return vec![i as i32, j as i32];
                    }
                }
            }
            return vec![];
        }

        // General Path: N > 32 (O(N) HashMap to prevent quadratic scaling)
        let mut map = HashMap::with_capacity(n);
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&prev_index) = map.get(&complement) {
                return vec![prev_index as i32, i as i32];
            }
            map.insert(num, i);
        }

        vec![]
    }
}
*/