// Last updated: 6/6/2025, 3:31:31 PM
impl Solution 
{
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool 
    {
        arr.into_iter()
            .scan(0, |c, i| 
            {
                // *c *= num & 1;
                // *c += num & 1;

                *c = if (i % 2 == 0) { 0 } else { *c + 1 } ;
                Some(*c)
            })
            .any(|c| c == 3)
    }
}