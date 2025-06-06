// Last updated: 6/6/2025, 3:31:17 PM
use std::collections::HashMap;

impl Solution 
{
    pub fn longest_palindrome(words: Vec<String>) -> i32 
    {
        let mut counts: HashMap<String, i32> = HashMap::new();
        for w in words.into_iter() { *counts.entry(w).or_insert(0) += 1; }

        let (mut word_count, mut central) = (0, false);
        for (w, &count) in counts.iter() 
        {
            let ch: Vec<char> = w.chars().collect();
            let (c1, c2) = (ch[0], ch[1]);

            // Palindromic
            if c1 == c2 
            {
                word_count += (count / 2) * 2;
                if count % 2 == 1 { central = true; }
            }

            // Non-Palindromic (enforce one direction ie. 'ab', not 'ba')
            else if c1 < c2 
            {
                let inv_w = format!("{}{}", c2, c1);
                if let Some(&inv_count) = counts.get(&inv_w) { word_count += 2 * std::cmp::min(count, inv_count); }
            }
        }

        if central { word_count += 1; }

        word_count * 2                                                          // (2 char / word)
    }
}