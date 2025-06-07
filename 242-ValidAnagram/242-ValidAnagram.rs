// Last updated: 6/7/2025, 2:41:55 AM
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool
    // {
    //     // Frequency Counter (Array/Vec): O(N) / O(1)
    //     if s.len() != t.len() { return false; }

    //     let mut counter: Vec<i32> = vec![0; 26];                                // Initialize a vector of 26 zeros
    //     let (sb, tb) = (s.as_bytes(), t.as_bytes());

    //     for i in 0..s.len() 
    //     {
    //         counter[(sb[i] - b'a') as usize] += 1;
    //         counter[(tb[i] - b'a') as usize] -= 1;
    //     }
        
    //     for &count in counter.iter() { if count != 0 { return false; } }        // Check if all counts in the counter are zero

    //     true
    // }

    // {
    //     // Sorting Characters: O(N log N) / O(1)
    //     if s.len() != t.len() { return false; }

    //     let (mut sc, mut tc): (Vec<char>, Vec<char>) = (s.chars().collect(), t.chars().collect()); 

    //     sc.sort_unstable();
    //     tc.sort_unstable();

    //     sc == tc
    // }

    { 
        // Frequency Table with Early Exit: O(N) / O(1)
        if s.len() != t.len() { return false; }

        let mut table: Vec<i32> = vec![0; 26]; // Frequency table for 26 lowercase letters

        // Add all max frequencies first , and then subtract in one loop to terminate early
        for cs in s.as_bytes() { table[(cs - b'a') as usize] += 1; }

        // Decrement counts for string t and check for negative counts
        for ct in t.as_bytes() 
        {
            let index = (ct - b'a') as usize;
            table[index] -= 1;
            if table[index] < 0 { return false; }
        }

        true
    }
}
