// Last updated: 6/6/2025, 3:32:09 PM
impl Solution 
{
    pub fn count_bits(n: i32) -> Vec<i32> 
    {
        fn pop_count(mut n: i32) -> i32 
        {
            let mut count = 0;
            while n != 0 
            {
                // Clear the least significant 1-bit
                n &= n - 1; 
                count += 1;
            }
            count
        }

        let mut ans = Vec::with_capacity((n + 1) as usize);
        for x in 0..=n { ans.push(pop_count(x)); }
        ans
    }
}
