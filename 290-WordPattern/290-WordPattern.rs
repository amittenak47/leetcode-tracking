// Last updated: 6/7/2025, 2:41:46 AM
use std::collections::HashMap;

impl Solution
{
    pub fn word_pattern(pattern: String, s: String) -> bool 
    // {
    //     // Two HashMaps: O(N) / O(N)
    //     let words: Vec<&str> = s.split(' ').collect();                                      // split string into vec by whitespace ' '

    //     if pattern.len() != words.len() { return false; }

    //     let mut cw: HashMap<char, &str> = HashMap::new();                                   // Bidirectional Mapping (char --> word and word --> char)
    //     let mut wc: HashMap<&str, char> = HashMap::new();

    //     for (pc, word) in pattern.chars().zip(words.iter().copied()) 
    //     {
    //         if let Some(mapping) = cw.get(&pc) { if *mapping != word { return false; } }    // Check if existing char --> word mapping is correct 
    //         else 
    //         {
    //             if wc.contains_key(word) { return false; }                                  // Check if existing word --> char has a reverse mapping for word

    //             cw.insert(pc, word);                                                        // Bidirectional Mapping
    //             wc.insert(word, pc);
    //         }
    //     }
    //     true
    // }

    {
        // Compare First Occurrence Indices: O(N) / O(N)
        let words: Vec<&str> = s.split(' ').collect();                                          // split string into vec by whitespace ' '

        if pattern.len() != words.len() { return false; }

        let mut cid_map: HashMap<char, usize> = HashMap::new();
        let mut wid_map: HashMap<&str, usize> = HashMap::new();

        for (i, (pc, word)) in pattern.chars().zip(words.iter().copied()).enumerate()           // Just assign the first index to each unique word / char and it should match, else return false
        {
            
            let cid = *cid_map.entry(pc).or_insert(i);
            let wid = *wid_map.entry(word).or_insert(i);
            
            if cid != wid { return false; }
        }
        true
    }
}