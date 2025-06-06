// Last updated: 6/6/2025, 3:31:00 PM
use std::{
    collections::HashSet,
};

impl Solution
{
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 
    {
        let mut st: HashSet<i32> = HashSet::new();
        for x in nums
        {
            if (x < k) { return -1; } // impossible
            else if (x > k) { st.insert(x); } // each unique is a level
        }
        st.len() as i32
    }
}