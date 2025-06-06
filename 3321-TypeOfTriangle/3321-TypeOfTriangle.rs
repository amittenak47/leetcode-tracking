// Last updated: 6/6/2025, 3:31:01 PM
impl Solution 
{
    pub fn triangle_type(nums: Vec<i32>) -> String 
    {
        let mut nums = nums;
        nums.sort();
        if nums[0] + nums[1] <= nums[2] { "none".to_string() }                              // valid triangle check
        else if nums[0] == nums[2] { "equilateral".to_string() }                            // sorted; hence i[0] == i[2]
        else if nums[0] == nums[1] || nums[1] == nums[2] { "isosceles".to_string() }        // transitive from prev step
        else { "scalene".to_string() }
    }
}