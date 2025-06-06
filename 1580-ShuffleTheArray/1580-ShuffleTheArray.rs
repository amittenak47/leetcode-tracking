// Last updated: 6/6/2025, 3:31:25 PM
impl Solution 
{
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> 
    {
        let (mut nums, nus) = (nums, n as usize);
        for i in nus..(2*nus)
        {
            let mut y = nums[i] << 10;          // Store y in upper 10 bits by shifting right
            nums[i - nus] |= y;                 // Store x in lower 10 bits (x OR y)
        }

        let lbmask: i32 = (1 << 10) - 1;

        // Iterate in reverse so as to avoid overwriting
        for i in (0..nus).rev() 
        {
            let (mut y, mut x) = (nums[i] >> 10, nums[i] & lbmask);
            nums[2 * i + 1] = y;
            nums[2 * i] = x;
        }
        nums
    }
}