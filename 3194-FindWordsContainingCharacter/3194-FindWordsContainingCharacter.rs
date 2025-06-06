// Last updated: 6/6/2025, 3:31:03 PM
use itertools::Itertools;

impl Solution 
{
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> 
    {
        words.iter()
            .positions(|word| word.contains(x))
            .map(|i| i as i32)
            .collect()
    }
}