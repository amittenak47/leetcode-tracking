// Last updated: 6/9/2025, 3:48:22 AM
impl Solution 
{
    pub fn calculate(s: String) -> i32 
    {
        let mut left_to_right: bool = true;

        if left_to_right
        {
            if (s.is_empty()) { return 0; }
            
            let mut stack = Vec::new();
            let mut num = 0;
            let mut operation = '+';
            
            for (i, ch) in s.chars().enumerate() 
            {
                if (ch.is_ascii_digit()) { num = (num * 10) + (ch.to_digit(10).unwrap() as i32); }

                if (!ch.is_ascii_digit() && !ch.is_whitespace() || i == s.len() - 1) 
                {
                    match operation
                    {
                        '-' => { stack.push(-1 * num); },
                        '+' => { stack.push(num); } ,
                        '*' => { if let Some(top) = stack.pop() { stack.push(top * num); } },
                        '/' => { if let Some(top) = stack.pop() { stack.push(top / num); } },
                        _ => (),
                    }

                    operation = ch;
                    num = 0;
                }
            }
            
            stack.iter().sum()
        }
        else
        {
            if s.is_empty() { return 0; }

            let mut stack: Vec<i32> = Vec::new();

            let mut i = s.len();
            while i > 0 
            {
                i -= 1;
                let ch = s.chars().nth(i).unwrap();

                if ch.is_ascii_digit() 
                {
                    // 1. Parse full number from the right
                    let rld = i+1;
                    while i > 0 && s.chars().nth(i - 1).unwrap().is_ascii_digit() { i -= 1; }

                    let mut rnum: i32 = s[i..rld].parse().unwrap();

                    // 2. Handle higher precedence operators '*' and '/'
                    loop
                    {
                        // 3. Search for operator, while next number is whitespace, until 'start' is reached or "*/" is not found)
                        let mut opi = i;
                        while opi > 0 && s.chars().nth(opi - 1).unwrap().is_whitespace() { opi -= 1; }
                        if opi == 0 || !"*/".contains(s.chars().nth(opi - 1).unwrap()) { break; }
                        
                        let op = s.chars().nth(opi - 1).unwrap();
                        
                        // 4. Search for left number, while next number is whitespace
                        i = opi - 1;
                        while i > 0 && s.chars().nth(i - 1).unwrap().is_whitespace() { i -= 1; }

                        let mut lld = i;    // lld is already (i + 1)
                        while i > 0 && s.chars().nth(i - 1).unwrap().is_ascii_digit() { i -= 1; }
                        
                        let lnum: i32 = s[i..lld].parse().unwrap();

                        // 5. Calculate and update rnum before looping again
                        if op == '*' { rnum = lnum * rnum; } 
                        else { rnum = lnum / rnum; } 

                    }

                    // 6. Handle lower precedence operators '+' and '-'
                    let mut opi = i;
                    while opi > 0 && s.chars().nth(opi - 1).unwrap().is_whitespace() { opi -= 1; }

                    if opi == 0 || s.chars().nth(opi - 1).unwrap() == '+' { stack.push(rnum); } 
                    else { stack.push(-rnum); }

                    // 7. Update the index
                    i = if opi > 0 { opi - 1 } else { 0 };
                }
            }

            // Add the final calculation and return value for the function.
            stack.iter().sum()
        }

    }
}