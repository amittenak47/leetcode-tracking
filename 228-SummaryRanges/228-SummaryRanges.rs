// Last updated: 6/7/2025, 2:41:58 AM
use std::vec;
use itertools::Itertools;

impl Solution 
{
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> 
    // {
    //     let mut ranges: Vec<String> = Vec::new();
    //     let n = nums.len();
    //     if (n == 0) { return ranges; }

    //     let mut i = 0;
    //     while i < n
    //     {   // can't use for-loop because our loop updates the range. 
    //         let mut start = nums[i];
    //         let mut j = i;

    //         // Expand range until the next element is outside of the current range
    //         while (j+1 < nums.len()) && (nums[j]+1 == nums[j+1]) { j += 1; }

    //         if (start != nums[j]) { ranges.push(start.to_string() + "->" + &nums[j].to_string()); } 
    //         else { ranges.push(start.to_string()); }

    //         i = j+1;
    //     }

    //     ranges
    // }
    {
        if (nums.is_empty()) { return Vec::new(); }
    
        nums.iter()
            .enumerate()
            .chunk_by(|&(i, &v)| v - i as i32)
            .into_iter()
            .map(|(k, g)| 
            {
                let gv: Vec<i32> = g.map(|(_, v)| *v).collect();

                let (start, end) = (*gv.first().unwrap(), *gv.last().unwrap());

                if (start == end) { start.to_string() }
                else { format!("{}->{}", start, end) }
            })
            .collect()
    }
}