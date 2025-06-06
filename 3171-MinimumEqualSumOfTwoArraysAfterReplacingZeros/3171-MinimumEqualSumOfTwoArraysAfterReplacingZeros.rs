// Last updated: 6/6/2025, 3:31:03 PM
impl Solution 
{
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 
//     {
//         let mut sum1: i64 = 0;
//         let mut sum2: i64 = 0;
//         let mut zero1 = 0;
//         let mut zero2 = 0;

//         for &x in &nums1 
//         {
//             sum1 += x as i64;
//             if x == 0 {
//                 sum1 += 1;
//                 zero1 += 1;
//             }
//         }

//         for &x in &nums2 
//         {
//             sum2 += x as i64;
//             if x == 0 {
//                 sum2 += 1;
//                 zero2 += 1;
//             }
//         }

//         if (zero1 == 0 && sum2 > sum1) || (zero2 == 0 && sum1 > sum2) {
//             return -1;
//         }

//         sum1.max(sum2)
//     }
    {
        let proc = |nums: &Vec<i32>| -> (i64, i64) 
        {
            nums.iter().fold((0, 0), |(sum, zeros), &val| 
            {
                if val == 0 { (sum+1, zeros+1) } 
                else { (sum + val as i64, zeros) }
            })
        };

        let ((sum1, zero1), (sum2, zero2)) = (proc(&nums1), proc(&nums2));

        if (zero1 == 0 && sum1 < sum2) || (zero2 == 0 && sum2 < sum1) { return -1; }

        sum1.max(sum2)
    }
}