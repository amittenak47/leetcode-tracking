// Last updated: 6/6/2025, 3:30:52 PM
impl Solution 
{
    pub fn count_subarrays(nums: Vec<i32>) -> i32 
    {
        let (n, mut ans): (usize, i32) = (nums.len(), 0);

        for i in 1..n-1 { ans = if (nums[i] == (nums[i-1] + nums[i+1]) * 2) {ans + 1} else {ans}; }

        ans
    }
}