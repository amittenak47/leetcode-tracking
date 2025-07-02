// Last updated: 7/1/2025, 7:20:58 PM
use std::collections::
{
    HashMap
};


impl Solution 
{
    pub fn is_valid(s: String) -> bool 
    {
        let mut stack: Vec<char> = Vec::new();
        let mut mappings = HashMap::new();
        mappings.insert(')', '(');
        mappings.insert('}', '{');
        mappings.insert(']', '[');

        for c in s.chars()
        {
            if let Some(&open) = mappings.get(&c)
            {
                let top = stack.pop().unwrap_or('~');
                if (top != open) { return false; }
            }
            else { stack.push(c); }
        }
        stack.is_empty()
    }
}