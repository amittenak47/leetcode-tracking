// Last updated: 6/7/2025, 2:42:10 AM
impl Solution {
    pub fn hamming_weight(n: i32) -> i32 
    {
        // count `1` bits using a mask (n /in [1, 2^32])
        let (mut cnt, mut mask) = (0, 1);
        for i in 0..=32
        {
            if (n & mask != 0) { cnt += 1; }
            mask <<= 1;
        }
        cnt
    }
}
