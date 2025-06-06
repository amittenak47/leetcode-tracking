// Last updated: 6/6/2025, 3:30:56 PM
impl Solution 
{
    // // Helper function for the binary search solution
    // fn isZeroArray(nums: &Vec<i32>, queries: &Vec<Vec<i32>>, k: usize) -> bool 
    // {
    //     let n = nums.len();
    //     if n == 0 { return true; } 
        
    //     // 1. Build difference array, with (n+1) marking end of difference interval
    //     let mut delta: Vec<i64> = vec![0; n + 1];
    //     for qi in 0..k 
    //     {
    //         if qi >= queries.len() { break; }

    //         let q = &queries[qi];
    //         let (start, end, v) = (q[0] as usize, q[1] as usize, q[2] as i64);

    //         if start < n { delta[start] += v; }
    //         if end + 1 < n + 1 { delta[end + 1] -= v; }
    //     }

    //     // 2. Ensure prefix sum >= nums[i] to ensure zero array can be formed
    //     let mut psum: i64 = 0;
    //     for i in 0..n 
    //     {
    //         psum += delta[i];
    //         if psum < nums[i] as i64 { return false; }
    //     }
    //     true
    // }

    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 
    // {
    //     // Binary Search for the min number of queries: O(log(M) * (N + M)) / O(N)
    //     let (N, Q) = (nums.len(), queries.len());
    //     if N == 0 { return 0; }

    //     // Check all queries first
    //     if !Self::isZeroArray(&nums, &queries, Q) { return -1; }

    //     let (mut low, mut high, mut mink) = (0 as usize, Q as usize, Q as usize);

    //     while low <= high 
    //     {
    //         let mid = low + (high - low) / 2;
    //         if Self::isZeroArray(&nums, &queries, mid) 
    //         {
    //             mink = mid;
    //             if mid == 0 { break; }
    //             high = mid - 1;
    //         } 
    //         else { low = mid + 1; }
    //     }
    //     mink as i32
    // }
    { 
        // Iterative Query Processing: O(N + M)
        // 1. Iterate through `nums`, and processes queries one by one until `nums[i]` is satisfied
        let N = nums.len();
        if N == 0 { return 0; }

        let mut delta: Vec<i64> = vec![0; N + 1];
        let mut psi: i64 = 0;                                         // track ps[i]
        let mut q: usize = 0;                                         // track query to avoid redundancy

        for i in 0..N 
        {
            psi += delta[i];                                          // Update ps[i] (+/- intervals added to delta)

            while psi < nums[i] as i64 
            {
                q += 1;                                         
                if q > queries.len() { return -1; }

                let query = &queries[q - 1];
                let (start, end, val) = (query[0] as usize, query[1] as usize, query[2] as i64);
                
                if end >= i                                           // (i-1, i-2, ...) satisfied; consider query if i \in [.., end]
                { 
                    let psi_start = std::cmp::max(start, i);          // (i-1, i-2, ...) satisfied; update delta for (i, i+1, i+2, ...)
                    
                    if psi_start < N { delta[psi_start] += val; }     // Update interval start in delta array
                    if end + 1 <= N { delta[end + 1] -= val; }        // Update interval end in delta array

                    if start <= i { psi += val; }                     // if i \in [start, ..] , update psi
                }
            }
        }
        q as i32
    }
}
