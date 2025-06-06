// Last updated: 6/6/2025, 3:32:04 PM
use::{
    std::collections::HashMap
};

impl Solution 
{
    pub fn can_construct(ransom_note: String, magazine: String) -> bool
    // { 
        
    //     // Two HashMaps Approach: O(magazine) / O(alphabet) or O(1)
    //     if ransom_note.len() > magazine.len() { return false; }                     // not enough char
    
    //     // Count Character Frequencies for given string: O(m)
    //     let getCharFreqCount = |s: &str| -> HashMap<char, i32> 
    //     {
    //         let mut counts = HashMap::new();
    //         for c in s.chars() { *(counts.entry(c)).or_insert(0) += 1; }
    //         counts
    //     };
    
    //     let rmap = getCharFreqCount(&ransom_note);                                  // use rmap to track char count of ransom_note
    //     let mmap = getCharFreqCount(&magazine);                                     // use mmap to track char count of magazine
    
    //     for (rc, rcount) in rmap.iter()                                             // Check if enough char in magazine for ransom_note
    //     {
    //         let mcount = mmap.get(rc).unwrap_or(&0);
    //         if mcount < rcount { return false; }
    //     }
    //     true
    // }

    // {
    //     // One HashMap Approach: O(magazine) / O(alphabet) or O(1)
    //     if ransom_note.len() > magazine.len() { return false; }                     // not enough char

    //     // Count Character Frequencies for given string: O(m)
    //     let getCharFreqCount = |s: &str| -> HashMap<char, i32> 
    //     {
    //         let mut counts = HashMap::new();
    //         for c in s.chars() { *(counts.entry(c)).or_insert(0) += 1; }
    //         counts
    //     };

    //     let mut mmap = getCharFreqCount(&magazine);

    //     for rc in ransom_note.chars() 
    //     {
    //         let mcount = mmap.entry(rc).or_insert(0);
    //         if *mcount == 0 { return false; }                                       // no char , no possible
    //         *mcount -= 1;
    //     }
    //     true
    // }

    { 
        // Using Sorted Vectors (Emulating Stacks)
        if ransom_note.len() > magazine.len() { return false; }                     // not enough char
    
        // Sort and Reverse character vector to access smallest character with .last() and .pop()
        let sortAndReverse = |s: &str| -> Vec<char> 
        {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable(); // Sorts ascending
            chars.reverse();       // Smallest char is now at the end of the vec
            chars
        };
    
        let (mut mvec, mut rvec) = (sortAndReverse(&magazine), sortAndReverse(&ransom_note));
    
        // Pop last char to advance mvec alone, or mvec and rvec when char matches, or false b/c rpeek char exceeds mpeek alphanum char
        while let (Some(&mpeek), Some(&rpeek)) = (mvec.last(), rvec.last()) 
        {
            if mpeek == rpeek { mvec.pop(); rvec.pop(); }
            else if mpeek < rpeek { mvec.pop(); } 
            else { return false; }
        }
    
        rvec.is_empty()
    }
}
