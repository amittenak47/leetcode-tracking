// Last updated: 6/6/2025, 3:31:15 PM
impl Solution 
{
    pub fn minimum_sum(mut num: i32) -> i32 
    {
        let mut nums = (0..4).fold([0; 4], |mut nums, i| 
        {
            nums[i] = num % 10;
            num /= 10;
            nums
        });
        nums.sort();
        (nums[0] * 10 + nums[2]) + (nums[1] * 10 + nums[3])                     // smallest 2 digits * 10: (20) + (20) + (3 + 9)
    }
}