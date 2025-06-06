// Last updated: 6/6/2025, 3:31:09 PM
impl Solution 
{
    pub fn remove_trailing_zeros(num: String) -> String 
    {
        // num.trim_end_matches('0').to_string()

        let nb = num.as_bytes();
        let mut j = nb.len();
        for i in (0..nb.len()).rev()
        {
            if (nb[i] != b'0') { j = i + 1; break; } 
        }

        String::from_utf8(nb[..j].to_vec()).unwrap()
    }
}