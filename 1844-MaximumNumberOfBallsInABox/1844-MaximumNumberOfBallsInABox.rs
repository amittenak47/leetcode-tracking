// Last updated: 6/6/2025, 3:31:23 PM
impl Solution 
{
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 
    // {
    //     let mut boxes = [0; 46];
    //     for mut num in low_limit..=high_limit 
    //     {
    //         let mut sum = 0;
    //         while num > 0 
    //         {
    //             sum += num % 10;
    //             num /= 10;
    //         }
    //         boxes[sum as usize] += 1;
    //     }
    //     boxes.into_iter().max().unwrap()
    // }
    {
        let mut boxes = [0; 46];
        (low_limit..=high_limit)
            .map(|mut num| 
            {
                let mut sum = 0;
                while num > 0 
                {
                    sum += num % 10;
                    num /= 10;
                }
                sum
            })
            .for_each(|sum| boxes[sum as usize] += 1);
        
        boxes.into_iter().max().unwrap()
    }
}