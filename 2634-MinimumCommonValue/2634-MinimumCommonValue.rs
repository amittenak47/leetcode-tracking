// Last updated: 6/6/2025, 3:31:11 PM


impl Solution 
{
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 
    {
        // let mut i = 0;
        // for v in nums1 
        // {
        //     while nums2[i] < v && i < nums2.len()-1 { i += 1; }
        //     if v == nums2[i] { return v; }
        // }
        // -1
        let (mut i, mut j) = (0, 0);
        while i < nums1.len() && j < nums2.len()
        {
            if nums1[i] == nums2[j] { return nums1[i]; }
            else if nums1[i] < nums2[j] { i += 1; }
            else { j += 1; }
        }
        -1
    }
}

/*
  Comparison of Approaches for getCommon (Finding Smallest Common in Sorted Arrays)
  
  let M = nums1.length
  let N = nums2.length

  M, N >= 1
  Arrays are sorted
  
  let S = min(M,N)
  let L = max(M,N)

+-----------------+-------------------------------+------------------+---------------------------------------------------------------------------------------------------+
| Approach        | Time Complexity               | Space Complexity | Notes / Key Scenarios & Strengths                                                                 |
+-----------------+-------------------------------+------------------+---------------------------------------------------------------------------------------------------+
| Two Pointers    | O(M+N)                        | O(1)             | - Generally best for sorted arrays (low constants, O(1) space).                                   |
|                 |                               |                  | - Effective runtime O(M) or O(N) if one array's values make the other pointer advance minimally.  |
|                 |                               |                  | - Full O(M+N) behavior (both M,N matter) when values are interleaved.                             |
+-----------------+-------------------------------+------------------+---------------------------------------------------------------------------------------------------+
| Binary Search   | O(S * log L)                  | O(1)             | - Best if S << L (one array much smaller). E.g., M=10, N=1M results in (10*log1M) vs 1M.          |
| (Iterate S,     |                               |                  | - O(1) space is a key advantage.                                                                  |
| Search L)       |                               |                  |                                                                                                   |
+-----------------+-------------------------------+------------------+---------------------------------------------------------------------------------------------------+
| Hash Table      | O(M+N) (Avg)                  | O(M) [1]         | - Good if arrays were not sorted (hypothetical for this problem). Robust average time.            |
| (Build from     | (Worst case rare, e.g. O(M*N))|                  | - Use if O(S) space is acceptable. Constant factors can be higher than Two Pointers.              |
| nums1,          |                               |                  |                                                                                                   |
| Probe nums2)    |                               |                  |                                                                                                   |
+-----------------+-------------------------------+------------------+---------------------------------------------------------------------------------------------------+
  [1] Space is O(M) if set built from nums1 (as per typical example code).
      Can be optimized to O(S) = O(min(M,N)) by building set from the smaller array.

Key Takeaways:
- Two Pointers: Top choice for sorted arrays needing O(1) space and best practical speed.
- Binary Search: Wins when one array is tiny and the other is huge, still O(1) space.
- Hash Table: Flexible (works for unsorted too), good average time, but uses extra space.
*/