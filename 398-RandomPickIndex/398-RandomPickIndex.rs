// Last updated: 6/6/2025, 3:32:01 PM
use std::collections::HashMap;

use rand::Rng;
use rand::seq::SliceRandom;

struct Solution 
{
    imap: HashMap<i32, Vec<i32>>,
    // nums: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution 
{

    fn new(nums: Vec<i32>) -> Self 
    {
        let mut imap: HashMap<i32, Vec<i32>> = HashMap::new();
        for i in 0..nums.len() { imap.entry(nums[i]).or_default().push(i as i32); }
        Solution { imap }

        // Solution { nums }
    }
    
    fn pick(&self, target: i32) -> i32 
    {
        let indices = self.imap.get(&target).unwrap();
        *indices.choose(&mut rand::thread_rng()).unwrap()

        // let mut rng = rand::thread_rng();
        // let (mut res, mut count) = (0, 0);

        // // for (i, &num) in self.nums.iter().enumerate()
        // for i in 0..self.nums.len()
        // {
        //     if self.nums[i] != target { continue; }

        //     count += 1;
        //     if rng.gen_range(0..count) == 0 { res = i as i32; }             // (rand() in 1/k) % count == 0
        // }

        // res
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: i32 = obj.pick(target);
 */