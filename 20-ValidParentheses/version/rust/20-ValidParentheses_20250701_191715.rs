// Last updated: 7/1/2025, 7:17:15 PM
use std::collections::
{
    HashMap,
    VecDeque
};


impl Solution 
{
    pub fn is_valid(s: String) -> bool 
    {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut mappings = HashMap::new();
        mappings.insert(')', '(');
        mappings.insert('}', '{');
        mappings.insert(']', '[');

        for c in s.chars()
        {
            if let Some(&open) = mappings.get(&c)
            {
                let top = stack.pop_back().unwrap_or('~');
                if (top != open) { return false; }
            }
            else { stack.push_back(c); }
        }
        stack.is_empty()
    }
}