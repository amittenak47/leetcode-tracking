// Last updated: 6/6/2025, 3:31:08 PM
use std::
{
    collections::HashMap,
    cmp::max,
};

impl Solution 
{
    pub fn maximize_greatness(nums: Vec<i32>) -> i32 
    // {
    //     let mut nums = nums.clone();
    //     nums.sort();

    //     let mut res: i32 = 0;
    //     for &i in &nums { if (i > nums[res as usize]) { res += 1 }; }

    //     res
    // }

    // {
    //     let mut count: HashMap<i32, i32> = HashMap::new();

    //     let mut k: i32 = 0;
    //     for &i in &nums 
    //     { 
    //         let current = count.entry(i).or_insert(0);
    //         *current += 1;
    //         k = max(k, *current);
    //     }
    //     nums.len() as i32 - k
    // }

    {
        let mut nums = nums;
        nums.sort_unstable();

        let (mut res, n, mut p1, mut p2): (i32, usize, usize, usize) = (0, nums.len(), 0, 0);
        while p2 < n
        {
            if nums[p2] > nums[p1] { res += 1; p1 += 1; }
            p2 += 1;
        }
        res
    }
}