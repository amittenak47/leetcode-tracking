// Last updated: 6/6/2025, 3:31:18 PM
use std::collections::HashMap;

impl Solution 
{
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> 
    {
        // 1. Count each digit in digits and store in a HashMap
        // 2. Iterate over all even 3-digit numbers in [100, 999]
        // 3. Count digits in each iteration and compare iteration freq count to digits freq count
        let mut res = Vec::new();
        let mut freq = HashMap::new();

        // Count the number of occurrences of each number in the integer array
        for &d in &digits { *freq.entry(d).or_insert(0) += 1; }
        
        // Enumerate all three-digit even numbers
        for i in (100..1000).step_by(2) 
        {
            let mut freq1 = HashMap::new();
            let mut num = i;
            
            // Count the frequency of each digit of the current even number
            while num > 0 
            {
                let d = num % 10;
                *freq1.entry(d).or_insert(0) += 1;
                num /= 10;
            }
            
            // Check if the conditions are met
            let mut cond = true;
            for (&d, &count) in &freq1 
            {
                if freq.get(&d).unwrap_or(&0) < &count 
                {
                    cond = false;
                    break;
                }
            }

            if cond { res.push(i); }
        }
        res
    }
}