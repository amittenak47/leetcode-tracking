// Last updated: 6/7/2025, 2:42:13 AM
impl Solution 
{
    pub fn convert_to_title(column_number: i32) -> String 
    {
        let mut n = column_number;
        let mut res = String::new();

        while n > 0
        {
            n -= 1; // convert from 1-based col number to 0-based index
            res.push(((n % 26) as u8 + b'A') as char);
            n /= 26;
        }
        res.chars().rev().collect::<String>()
    }
}