// Last updated: 6/6/2025, 3:31:21 PM
impl Solution 
{
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> 
    {
        let mut nums = nums;
        let mut n = nums.len();

        // constraint: nums elements in [1, 1000]
        // Store nums[i] in the upper digits (outside of constraints)
        for i in 0..n
        {
            nums[i] += 10000 * (nums[nums[i] as usize] % 10000);
        }

        for i in 0..n { nums[i] /= 10000; }
        nums
    }
}