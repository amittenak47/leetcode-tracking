// Last updated: 6/6/2025, 3:30:58 PM
use std::
{
    cmp::Reverse, 
    mem::replace
};

impl Solution 
{
    pub fn button_with_longest_time(events: Vec<Vec<i32>>) -> i32 
    {
        // 1. Use iter() and scan() to get the max((e[1] - ps)), where ps is the previous state, replaced by e[1] at each step
        // 2. Wrap e[0] (button ID) in Reverse() so that max() selects the smallest button ID value
        // 3. Map Option<tuple>: | (_, Reverse(i)) | i and unwrap() Option
        events
            .into_iter()
            .scan(0, | ps, e | Some((e[1] - replace(ps, e[1]), Reverse(e[0]))))
            .max()
            .map( | (_, Reverse(i)) | i)
            .unwrap_or_else(|| 0)
    }
}