// Last updated: 6/6/2025, 3:30:54 PM
impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> 
    {
        nums.iter()
            .filter(|&i| i % 2 == 1)
            .enumerate()
            .fold(vec![0; nums.len()], |mut acc, (i, _)| 
            {
                acc[(nums.len()-1) - i] = 1;
                acc
            })
    }
}