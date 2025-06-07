// Last updated: 6/7/2025, 2:42:00 AM
use std::{
    collections::HashSet,
    collections::BTreeSet,
};

impl Solution 
{
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool 
    {

        let mut set: HashSet<i32> = HashSet::new();                             // O(N) / O(min(n, k) b/c k elements in window
        // let mut set: BTreeSet<i32> = BTreeSet::new();                        // O(N log(min(n, k))) / O(min(n, k) b/c k elements in window 

        for i in 0..nums.len()
        {
            // Check if duplicate already exists in set; 
            // Because we maintain a window of size k, any duplicate element will immediately return true in the k + 1 position
            // as it falls within the abs(i - j) <= (k = 3) constraint which is 0-indexed
            // If the element is not a duplicate, then the first element is removed so that this window is maintained
            if (set.contains(&nums[i])) { return true; }
            set.insert(nums[i]);

            if (set.len() > (k as usize)) { set.remove(&nums[i - (k as usize)]); }
        }
        false
    }
}