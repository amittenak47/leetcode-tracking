// Last updated: 6/6/2025, 3:31:22 PM
use std::
{
    cmp::max,
    vec,
};

impl Solution 
{
    pub fn merge_alternately(word1: String, word2: String) -> String 
    {
        let (mut w1, mut w2) = (word1.as_bytes(), word2.as_bytes()); 
        let (mut m, mut n, mut res) = (w1.len(), w2.len(), Vec::new());

        for i in 0..max(m, n)
        {
            if (i < m) { res.push(w1[i]); }
            if (i < n) { res.push(w2[i]); }
        }

        String::from_utf8(res).unwrap()
    }
}