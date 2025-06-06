// Last updated: 6/6/2025, 3:31:05 PM
impl Solution 
{
    pub fn get_words_in_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> 
    {

        // Bottom Up DP
        let n = groups.len();
        if n == 0 { return Vec::new(); }

        let mut last = 0;
        let mut dp: Vec<(i32, i32)> = vec![(1, -1); n];

        for i in 1..n 
        {
            for j in 0..i 
            {
                // 1. Only expand prior tabulation if it is larger than current expansion
                // 2. groups[i] != groups[j]
                // 3. Hamming Distance == 1 and words[i].len() == words[j].len()
                if dp[j].0 + 1 > dp[i].0 && groups[i] != groups[j] && Self::isHam(&words[i], &words[j])
                {
                    dp[i].0 = dp[j].0 + 1;
                    dp[i].1 = j as i32;
                }
            }
            if dp[i].0 > dp[last].0 { last = i; }
        }
        let mut ans = Vec::new();
        let mut bt = last as i32;
        while bt >= 0 
        {
            ans.push(words[bt as usize].clone());
            bt = dp[bt as usize].1;
        }

        ans.reverse();
        ans
    }

    fn isHam(s1: &String, s2: &String) -> bool 
    {
        if s1.len() != s2.len() { return false; }
        
        let mut ham = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) 
        {
            if c1 != c2 { ham += 1; if ham > 1 { return false; } }
        }
        ham == 1
    }
}