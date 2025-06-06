// Last updated: 6/6/2025, 3:30:55 PM
use std::collections::BinaryHeap;

impl Solution 
{
    pub fn max_removal(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 
    { 
        // Priority Queue: O(N + M * log M) / O(N + M)
        let mut queries = queries;
        queries.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut heap = BinaryHeap::new();
        let mut delta = vec![0; nums.len() + 1];
        let mut ops = 0;
        let mut j = 0;
        
        for i in 0..nums.len() 
        {
            ops += delta[i];
            while j < queries.len() && queries[j][0] == i as i32 
            {
                heap.push(queries[j][1]);
                j += 1;
            }
            while ops < nums[i] && !heap.is_empty() && *heap.peek().unwrap() >= i as i32 
            {
                ops += 1;
                let end = heap.pop().unwrap() as usize;
                delta[end + 1] -= 1;
            }

            if ops < nums[i] { return -1; }
        }
        heap.len() as i32
    }
}