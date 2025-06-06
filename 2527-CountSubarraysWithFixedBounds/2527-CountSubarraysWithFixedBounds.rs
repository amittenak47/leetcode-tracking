// Last updated: 6/6/2025, 3:31:11 PM
impl Solution 
{
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 
    {
        let (mut answer, mut min_pos, mut max_pos, mut left): (i64, i32, i32, i32) = (0, -1, -1, -1);   
    
        for i in 0..nums.len()
        {
            if (nums[i] < min_k || nums[i] > max_k) { left = i as i32; }

            if (nums[i] == min_k) { min_pos = i as i32; }
            if (nums[i] == max_k) { max_pos = i as i32; }

            answer = answer + (std::cmp::max(0, std::cmp::min(min_pos, max_pos) - left) as i64);
        }

        answer
    }
}
