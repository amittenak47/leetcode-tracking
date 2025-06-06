// Last updated: 6/6/2025, 3:31:43 PM
impl Solution 
{
    pub fn num_tilings(n: i32) -> i32 
    // {
    //     let MOD: i64 = 1000000007;
    //     if (n <= 2) { return n; }
        
    //     let mut dp3: i64 = 5;               // dp[3] = 5
    //     let mut dp2: i64 = 2;               // dp[2] = 2
    //     let mut dp1: i64 = 1;               // dp[1] = 1

    //     for k in 4..(n + 1) 
    //     {
    //         let mut dp4 = 0;
    //         let mut tdp1: i64 = dp2;
    //         dp2 = dp3; 

    //         // dp[i] =  2 * dp[i-1]  + dp[i-3]          % MOD
    //         // dp[4] =  2 * dp[3]    + dp[1]            % MOD
    //         dp4      = (2 * dp3      + dp1)             % MOD;

    //         // dp[1] in next iteration
    //         dp1 = tdp1;
    //         dp3 = dp4;
    //     }
    //     dp3 as i32
    // }

    // Given:
    // f[k] = f[k-1] + f[k-2] + 2 * p[k-1]
    // p[k] = p[k-1] + f[k-2]

    // Derivation:
    // 2 * p[k-1] = f[k] - f[k-1] - f[k-2]

    // p[k-1] = p[k-2] + f[k-3]

    // 2 * (p[k-2] + f[k-3]) = f[k] - f[k-1] - f[k-2]
    // 2 * p[k-2] + 2 * f[k-3] = f[k] - f[k-1] - f[k-2]

    // 2 * p[k-2] = f[k-1] - f[k-2] - f[k-3]

    // (f[k-1] - f[k-2] - f[k-3]) + 2 * f[k-3] = f[k] - f[k-1] - f[k-2]
    // f[k-1] - f[k-2] + f[k-3] = f[k] - f[k-1] - f[k-2]
    // f[k] = f[k-1] - f[k-2] + f[k-3] + f[k-1] + f[k-2]
    // f[k] = 2 * f[k-1] + f[k-3]

    // f[k] = (2 * f[k-1] + f[k-3]) % MOD

    {
        let MOD: i64 = 1000000007;
        // handle base case scenarios
        if (n <= 2) { return n; }
        let n = n as usize;

        // f[k]: number of ways to "fully cover a board" of width k
        let mut f: Vec<i64> = vec![0; n + 1];

        // p[k]: number of ways to "partially cover a board" of width k
        let mut p: Vec<i64> = vec![0; n + 1];

        // initialize f and p with results for the base case scenarios
        f[1] = 1;
        f[2] = 2;
        p[2] = 1;

        for k in 3..(n + 1) 
        {
            f[k] = (f[k - 1] + f[k - 2] + 2 * p[k - 1]) % MOD;
            p[k] = (p[k - 1] + f[k - 2]) % MOD; 
        }

        f[n] as i32
    }
}