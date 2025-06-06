// Last updated: 6/6/2025, 3:31:04 PM
use::std::vec;

impl Solution 
{
    // pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> 
    // {
    //     let mut ans: Vec<String> = Vec::new();
    //     ans.push(words[0].clone());

    //     for i in 1..words.len() { if (groups[i] != groups[i - 1]) { ans.push(words[i].clone()); } }
    //     ans         
    // }

    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> 
    {
        let n = words.len();
        if (n == 0) { return Vec::new(); }

        let mut last = 0;
        let mut dp: Vec<(i32, i32)> = vec![(1, -1); n];

        for i in 1..n 
        {
            for j in (0..i).rev()
            {
                if groups[i] != groups[j] && dp[j].0 + 1 > dp[i].0 
                {
                    dp[i].0 = dp[j].0 + 1;
                    dp[i].1 = j as i32;
                }
            }
            if dp[i].0 > dp[last].0 { last = i; }
        }

        let mut res = Vec::new();
        let mut bt = last as i32;
        while bt != -1 
        {
            res.push(words[bt as usize].clone());
            bt = dp[bt as usize].1;
        }
        
        res.reverse();
        res
    }
}