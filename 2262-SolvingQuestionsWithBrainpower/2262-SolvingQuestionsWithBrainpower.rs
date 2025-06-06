// Last updated: 6/6/2025, 3:31:16 PM
impl Solution 
{
    pub fn dfs(dp: &mut Vec<i64>, questions: &Vec<Vec<i32>>, dough: usize) -> i64
    {
        if dough >= questions.len() { return 0 as i64; }
        if (dp[dough] != 0) { return dp[dough] as i64; }

        let mut points = questions[dough][0] as i64;
        let mut skippy = questions[dough][1] as usize;

        // (solve, skip)
        dp[dough] = std::cmp::max(points + Self::dfs(dp, questions, (dough + skippy + 1) as usize), Self::dfs(dp, questions, (dough + 1) as usize));
        dp[dough]
    }

    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 
    // // DP (i -> i + skip + 1) in reverse
    // {
    //     let n = questions.len();
    //     if (n == 1) { return questions[0][0] as i64; }
        
    //     let mut dp: Vec<i64> = vec![0; n];
    //     dp[n - 1] = questions[n - 1][0] as i64;

    //     for i in (0..=(n-2)).rev()
    //     {
    //         dp[i] = questions[i][0] as i64;

    //         let skip = questions[i][1] as usize;
    //         if (i + skip + 1 < n) { dp[i] += dp[i + skip + 1]; } // bounds check; already tabulized

    //         dp[i] = dp[i].max(dp[i + 1]); // max(solve, skip)
    //     }

    //     dp[0] as i64
    // }

    {
        let n = questions.len();
        let mut dp: Vec<i64> = vec![0; n];
        return Self::dfs(&mut dp, &questions, 0);
    }


}