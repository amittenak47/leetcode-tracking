// Last updated: 6/6/2025, 3:31:20 PM
impl Solution 
{
    pub fn reverse_prefix(word: String, ch: char) -> String 
    {
        // word.split_once(ch)
        //     .map(|(l, r)| String::from_iter(l.chars().chain([ch]).rev()) + r)
        //     .unwrap_or(word)

        let mut wb = word.into_bytes();
        let (mut l, mut r) = (0, 0);

        while r < wb.len()
        {
            if (wb[r] as char == ch)
            {
                while (l < r)
                {
                    (wb[l], wb[r]) = (wb[r], wb[l]);
                    l += 1;
                    r -= 1;
                }
                return String::from_utf8(wb).unwrap();
            }
            r += 1;
        }

        String::from_utf8(wb).unwrap()
    }
}