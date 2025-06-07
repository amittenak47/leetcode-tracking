// Last updated: 6/7/2025, 3:11:46 PM
use std::cmp;

impl Solution 
{
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 
    {
        let (mut count, mut maxCount) = (0, 0);
        for i in 0..nums.len()
        {
            if (nums[i] == 1) { count += 1; } 
            else 
            {
                maxCount = std::cmp::max(maxCount, count);
                count = 0;
            }
        }
        
        std::cmp::max(maxCount, count)
    }
}