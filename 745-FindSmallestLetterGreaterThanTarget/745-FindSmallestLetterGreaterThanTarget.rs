// Last updated: 6/6/2025, 3:31:48 PM
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char 
    {
        let (mut left, mut right): (i32, i32) = (0, (letters.len() as i32) - 1);

        while (left <= right)
        {
            let mut mid = left + (right - left) / 2;
            if (letters[mid as usize] > target) { right = mid - 1; }
            else { left = mid + 1; }
        }

        if (left == (letters.len() as i32)) { letters[0] } else { letters[left as usize] }
    }
}
