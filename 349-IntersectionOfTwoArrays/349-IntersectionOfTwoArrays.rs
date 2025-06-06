// Last updated: 6/6/2025, 3:32:07 PM
use std::{
    collections::HashSet,
    iter::FromIterator, 
};

impl Solution 
{
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> 
    // {
    //     HashSet::<i32>::from_iter(nums1).intersection(&HashSet::<i32>::from_iter(nums2)).copied().collect()
    // }
    {
        // Sort both arrays
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        nums1.sort();
        nums2.sort();

        // Initialize two pointers
        let mut N = nums1.len();
        let mut M = nums2.len();
        let mut p1 = 0;
        let mut p2 = 0;
        
        // Create set that stores integers appearing in both arrays
        let mut intersection: HashSet::<i32> = HashSet::new();

        // Iterate the pointers from left to right
        while (p1 < N && p2 < M) 
        {
            // Add a value to the set if values at both pointers equal
            if (nums1[p1] == nums2[p2]) 
            {
                intersection.insert(nums1[p1]);
                p1 += 1;
                p2 += 1;
            }
            // Otherwise, increment the pointer of the smaller integer
            else if (nums1[p1] < nums2[p2]) { p1 += 1; }
            else { p2 += 1; }
        }

        // Convert intersection to an array
        let mut result: Vec<i32> = Vec::with_capacity(intersection.len());
        for x in intersection { result.push(x); }

        result
    }
}