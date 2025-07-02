// Last updated: 7/1/2025, 7:38:05 PM
use std::collections::VecDeque;

impl Solution 
{
    pub fn simplify_path(path: String) -> String 
    {
        let mut stack: Vec<String> = Vec::new();

        for s in path.split("/")
        {
            match s 
            {
                "" | "." => { continue; },
                ".." => { stack.pop(); },
                _ => { stack.push(s.to_string()); }

            }
        }

        if stack.is_empty() { "/".to_string() } 
        else { format!("/{}", stack.join("/")) }
    }
}

