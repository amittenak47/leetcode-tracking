// Last updated: 6/6/2025, 3:31:51 PM
impl Solution 
{
    pub fn valid_palindrome(s: String) -> bool 
    {
        let mut sb = s.as_bytes();
        let (mut i, mut j): (i32, i32) = (0, (s.len() as i32) - 1);
        
        while (i < j) 
        {
            // We only have to delete 1 char, so call check() to delete both chars and continue checking
            if (sb[i as usize] != sb[j as usize]) { return (Self::check(sb, i, j-1) || Self::check(sb, i+1, j)); }
            i += 1;
            j -= 1;
        }

        true
    }

    fn check(sb: &[u8], mut i: i32, mut j: i32) -> bool
    {
        while (i < j) 
        {
            if (sb[i as usize] != sb[j as usize]) { return false; }
            
            i += 1;
            j -= 1;
        }
        
        true
    }
}
