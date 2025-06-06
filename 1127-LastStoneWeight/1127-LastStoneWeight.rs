// Last updated: 6/6/2025, 3:31:34 PM
use std::{
    collections::BinaryHeap
};

impl Solution 
{

    // fn remove_largest(stones: &mut Vec<i32>) -> i32
    // {
    //     let (i, &weight) = stones.iter().enumerate().max_by_key(|&(_, &w)| w).unwrap();
    //     // println!("Remove {:?}", stones);
    //     stones.swap_remove(i);
    //     // println!("After {:?}\n", stones);
    //     weight
    // }

    // pub fn last_stone_weight(stones: Vec<i32>) -> i32 
    // {
    //     let mut stones = stones;

    //     while stones.len() > 1
    //     {
    //         let stone1 = Self::remove_largest(&mut stones);
    //         let stone2 = Self::remove_largest(&mut stones);
    //         if (stone1 != stone2) { stones.push(stone1 - stone2); }
    //         // println!("Main: {:?}", stones);
    //     }

    //     stones.pop().unwrap_or(0)
    // }

    pub fn last_stone_weight(stones: Vec<i32>) -> i32 
    {
        // Create a max-heap from the stones vector
        let mut heap: BinaryHeap<i32> = BinaryHeap::from(stones);
        
        // Process stones until 1 or 0 remain
        while heap.len() > 1 
        {
            let stone1 = heap.pop().unwrap(); // Largest stone
            let stone2 = heap.pop().unwrap(); // Second largest
            
            // If stones are unequal, push the remainder
            if stone1 != stone2 { heap.push(stone1 - stone2); }
        }
        
        // Return the last stone or 0 if empty
        heap.pop().unwrap_or(0)
    }
}