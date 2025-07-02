// Last updated: 7/1/2025, 7:32:18 PM
use std::collections::
{
    HashMap,
    VecDeque
};


impl Solution 
{
    // pub fn is_valid(s: String) -> bool 
    // {
    //     let mut stack: Vec<char> = Vec::new();
    //     let mut mappings = HashMap::new();
    //     mappings.insert(')', '(');
    //     mappings.insert('}', '{');
    //     mappings.insert(']', '[');

    //     for c in s.chars()
    //     {
    //         if let Some(&open) = mappings.get(&c)
    //         {
    //             let top = stack.pop().unwrap_or('~');
    //             if (top != open) { return false; }
    //         }
    //         else { stack.push(c); }
    //     }
    //     stack.is_empty()
    // }

    pub fn is_valid(s: String) -> bool 
    {
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in s.chars()
        {
            if (c == '(') || (c == '{') || (c == '[') { stack.push_back(c); }
            else 
            {
                if stack.is_empty() { return false; }
                match stack.back()
                {
                    Some(&top) => {
                        // Stack was not empty, got `top_char`
                        if (c == ')' && top != '(') ||
                           (c == '}' && top != '{') ||
                           (c == ']' && top != '[')
                        {
                            return false; // Mismatch found
                        }
                        // If it matches, continue the loop
                    },
                    None => return false,
                }
            stack.pop_back();
            }
        }
        stack.is_empty()
    }
}