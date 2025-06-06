// Last updated: 6/6/2025, 3:31:02 PM
use std::cmp::{max, min};

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64
    { 
        // Greedy (Sorting-based approach): O(n log n) / O(n)
        // 1. Calculate delta for each number
        // 2. Sort delta in descending order
        // 3. Add delta pairs that add (positively) to sum
        let mut sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let mut delta: Vec<i64> = nums.iter().map(|&v32| 
        {
            let v = v32 as i64;
            let xorv = (v32 ^ k) as i64;
            xorv - v
        }).collect();

        delta.sort_by(|a, b| b.cmp(a));                                 // Sort in decreasing order

        for i in (0..delta.len()).step_by(2) 
        {
            if i + 1 < delta.len()
            { 
                let pair = delta[i] + delta[i + 1];
                if pair > 0 { sum += pair; } 
                else { break; }                                         // pair_sum < 0, remaining pairs are negative
            }
        }
        sum
    }
}