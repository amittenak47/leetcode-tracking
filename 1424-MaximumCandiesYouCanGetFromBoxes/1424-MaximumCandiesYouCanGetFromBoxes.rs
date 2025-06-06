// Last updated: 6/6/2025, 3:31:27 PM
use std::collections::VecDeque;

impl Solution 
{
    pub fn max_candies(status: Vec<i32>, candies: Vec<i32>, keys: Vec<Vec<i32>>, contained_boxes: Vec<Vec<i32>>, initial_boxes: Vec<i32>) -> i32 
    {

        // 1. Note the boxes you can open with 'can_open' vec from 'status' vec
        // 2. Iterate over 'initial_boxes', tracking box in 'q' and noting 'has_box' vec
        //      - If 'can_open', note 'used' vec and total count of candies
        // 3. For each box in 'q', extend 'q' with boxes from 'key', noting 'has_box', and subsequently 'used' and 'can_open', as well 
        // as 'contained_boxes', updating total count of candies.
        
        // let n = status.len();
        // let mut can_open = vec![false; n];
        // let mut has_box = vec![false; n];
        // let mut used = vec![false; n];

        // for i in 0..n { can_open[i] = (status[i] == 1); }

        // let (mut count, mut q) = (0, VecDeque::new());
        // for box_id in initial_boxes 
        // {
        //     has_box[box_id as usize] = true;
        //     if can_open[box_id as usize] 
        //     {
        //         q.push_back(box_id);
        //         used[box_id as usize] = true;
        //         count += candies[box_id as usize];
        //     }
        // }

        // while let Some(big_box) = q.pop_front() 
        // {
        //     for &key in &keys[big_box as usize] 
        //     {
        //         can_open[key as usize] = true;
        //         if !used[key as usize] && has_box[key as usize]                             // Dungeon Key: Note if key is encountered, but box has never been seen before, we cannot profit but we can track it
        //         {
        //             q.push_back(key);
        //             used[key as usize] = true;
        //             count += candies[key as usize];
        //         }
        //     }

        //     // redundant container (do not reuse)
        //     for &box_id in &contained_boxes[big_box as usize] 
        //     {
        //         has_box[box_id as usize] = true;
        //         if !used[box_id as usize] && can_open[box_id as usize]                      // Dungeon Chest: Not if we can open box, we can profit from it
        //         {
        //             q.push_back(box_id);
        //             used[box_id as usize] = true;
        //             count += candies[box_id as usize];
        //         }
        //     }
        // }

        // count

        fn dfs(box_id: usize, count: &mut i32, has_box: &mut Vec<bool>, can_open: &mut Vec<bool>, used: &mut Vec<bool>, candies: &Vec<i32>, keys: &Vec<Vec<i32>>, contained_boxes: &Vec<Vec<i32>>)
        {
            if !has_box[box_id] || !can_open[box_id] || used[box_id] { return; }

            used[box_id] = true;
            *count += candies[box_id];

            for &key in &keys[box_id]
            {
                can_open[key as usize] = true;
                dfs(key as usize, count, has_box, can_open, used, candies, keys, contained_boxes);                           // if !used[key as usize] && has_box[key as usize] optional
            }

            for &cbox_id in &contained_boxes[box_id] 
            {
                has_box[cbox_id as usize] = true;
                dfs(cbox_id as usize, count, has_box, can_open, used, candies, keys, contained_boxes);                       // if !used[cbox_id as usize] && can_open[cbox_id as usize] optional
            }
        }

        let n = status.len();
        let mut can_open = vec![false; n];
        let mut has_box = vec![false; n];
        let mut used = vec![false; n];
        let mut count = 0;

        for i in 0..n { can_open[i] = (status[i] == 1); }
        for box_id in initial_boxes { has_box[box_id as usize] = true; }
        for i in 0..n { dfs(i, &mut count, &mut has_box, &mut can_open, &mut used, &candies, &keys, &contained_boxes); }
        
        count
    }
}

