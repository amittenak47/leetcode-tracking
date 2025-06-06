// Last updated: 6/6/2025, 3:32:10 PM
impl Solution 
{
    pub fn max_sub_array_len(nums: Vec<i32>, k: i32) -> i32 
    {
        let (mut ps, mut res): (i32, i32) = (0, 0);
        let mut indies: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

        for i in 0..nums.len() 
        {
            ps += nums[i];

            if (ps == k) { res = (i as i32) + 1; }

            if let Some(&earl) = indies.get(&(ps - k)) { res = res.max((i as i32) - (earl as i32)); }

            indies.entry(ps).or_insert(i);
        }

        res
    }
}
