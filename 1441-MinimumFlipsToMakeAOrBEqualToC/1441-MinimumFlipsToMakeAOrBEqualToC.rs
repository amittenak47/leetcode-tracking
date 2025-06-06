// Last updated: 6/6/2025, 3:31:28 PM
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 
    {
        let mut res = 0;
        let (mut a, mut b, mut c) = (a, b, c);

        while (a != 0 || b != 0 || c != 0)
        {
            let (mut abit, mut bbit, mut cbit) = (a & 1, b & 1, c & 1);

            if (cbit==0) { res += abit + bbit; }
            else { res += if (abit==0 && bbit==0) { 1 } else { 0 }; }
            
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        res
    }
}