// Last updated: 6/6/2025, 3:31:29 PM
// Cargo.toml
// [dependencies]
// linked-hash-map = "0.5"

use std::collections::{VecDeque, HashMap};
// use linked_hash_map::LinkedHashSet;

struct FirstUnique 
{
    queue: VecDeque<i32>,
    is_unique: HashMap<i32, bool>,
    // ordered_uniques: LinkedHashSet<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FirstUnique 
{
    // Brute force is to iterate over each item in the queue, and return the first element with count == 1
    // Constructor = O(N)
    // Add() = O(1)
    // ShowFirstUnique = O(N*M), N: queue length; M: average cost of count

    // Constructor = O(N)
    // Add() = O(1)
    // ShowFirstUnique = O(1) amortized, proportional to number of Add() calls
    fn new(nums: Vec<i32>) -> Self 
    {
        let mut fu = FirstUnique 
        {
            queue: VecDeque::new(),
            is_unique: HashMap::new(),
            // set: LinkedHashSet::new(),
        };

        for num in nums { fu.add(num); }
        fu
    }
    
    fn show_first_unique(&mut self) -> i32 
    {
        // Clean non-unique elements from the front of the queue
        while let Some(&front) = self.queue.front() 
        {
            if let Some(is_u) = self.is_unique.get(&front) 
            {
                if !*is_u { self.queue.pop_front(); }
                else { return front; }
            } 
        }
        // Return -1 as unique doesn't exist
        -1

        // if let Some(&first) = self.ordered_uniques.front() { first }
        // else { -1 }
    }
    
    // `&mut self` because it modifies the queue and the hashmap.
    fn add(&mut self, value: i32) 
    {
        // // Non-unique value
        // if self.is_unique.contains_key(&value) 
        // {
        //     if let Some(status) = self.is_unique.get_mut(&value) { *status = false; }
        // }

        // // New unique value 
        // else
        // {
        //     self.is_unique.insert(value, true);
        //     self.queue.push_back(value);
        // }
        match self.is_unique.get_mut(&value) 
        {
            Some(is_u) => 
            {
                if *is_u { *is_u = false; } // self.set.remove(&value); }
            }
            None => 
            {
                self.is_unique.insert(value, true);
                self.queue.push_back(value);
                // self.set.insert(value);
            }
        }
    }
}

/**
 * Your FirstUnique object will be instantiated and called as such:
 * let obj = FirstUnique::new(nums);
 * let ret_1: i32 = obj.show_first_unique();
 * obj.add(value);
 */