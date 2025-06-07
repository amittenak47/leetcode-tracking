// Last updated: 6/7/2025, 2:42:05 AM
use std::{collections::HashMap,};

impl Solution 
{
    pub fn is_isomorphic(s: String, t: String) -> bool 
    {
        // Two HashMaps for Character Mappings: O(N) / O(K=26)
        if s.len() != t.len() { return false; }

        let mut stmap: HashMap<char, char> = HashMap::new();
        let mut tsmap: HashMap<char, char> = HashMap::new();

        // Iterate over the characters of both strings simultaneously.
        for (cs, ct) in s.chars().zip(t.chars()) 
        {
            match (stmap.get(&cs), tsmap.get(&ct)) 
            {
                (Some(tt), Some(ss)) => { if *tt != ct || *ss != cs { return false; } }
                (Some(tt), None) => { return false; }
                (None, Some(ss)) => { return false; }
                (None, None) => 
                {
                    stmap.insert(cs, ct);
                    tsmap.insert(ct, cs);
                }
            }
        }

        true
    }

    // { 
    //     // Transform String to Canonical Form: O(N) / O(N)
    
    //     // Replace each character with the index of its first occurrence
    //     let numerical = |input_str: &str| -> String 
    //     {
    //         let mut map: HashMap<char, usize> = HashMap::new();
    //         let mut res: Vec<String> = Vec::new();
    //         let mut last = 0;

    //         for i in input_str.chars() 
    //         {
    //             let mapping = map.entry(i).or_insert_with(|| last);
    //             last += 1;
    //             res.push(mapping.to_string());
    //         }
    //         res.join(", ") // 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 2 10 == 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 21 0 lol
    //     };

    //     if s.len() != t.len() { return false; }

    //     // numerical(&s) == numerical(&t)

    //     let S = numerical(&s);
    //     let T = numerical(&t);
    //     println!("{} == {}", S, T);
    //     S == T

    // }
}