// Last updated: 6/6/2025, 3:30:56 PM
impl Solution 
{
    // Select a subset --> {} or [l, r] 
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool 
    {

        let mut sum = 0;
        queries
            .into_iter()
            .fold(vec![0; nums.len() + 1], |mut delta, q| 
            {
                delta[q[0] as usize] += 1;
                delta[q[1] as usize + 1] -= 1;
                delta
            })
            .into_iter()
            .zip(nums)
            .all(|(ps, num)| 
            {
                sum += ps;
                num <= sum
            })

        // // Track delta ops for each interval
        // let mut delta = vec![0; nums.len() + 1];
        // for q in queries 
        // {
        //     let (l, r) = (q[0] as usize, q[1] as usize);
        //     delta[l] += 1;
        //     delta[r + 1] -= 1;
        // }

        // // Create prefix sum from delta to track operations between intervals
        // let mut sum = 0;
        // let mut ps = Vec::with_capacity(delta.len());
        // for d in delta 
        // {
        //     sum += d;
        //     ps.push(sum);
        // }

        // // If ps[i] < nums[i] , not enough ops at index to become 0-array
        // for i in 0..nums.len() { if ps[i] < nums[i] { return false; } }
        // true
    }
}