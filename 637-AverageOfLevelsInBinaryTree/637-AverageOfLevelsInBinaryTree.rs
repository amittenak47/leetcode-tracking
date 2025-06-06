// Last updated: 6/6/2025, 3:31:53 PM
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
use std::vec;
use std::collections::VecDeque;

impl Solution 
{
    // pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> 
    // {
    //     let Some(root_node) = root else { return Vec::new(); };
        
    //     let mut level_counts: Vec<i32> = Vec::new();
    //     let mut level_sums: Vec<i64> = Vec::new();

    //     Self::average(root_node, 0, &mut level_sums, &mut level_counts);
        
    //     let mut res: Vec<f64> = Vec::new();
    //     for i in 0..level_sums.len() 
    //     {
    //         if level_counts[i] > 0 
    //         {
    //             let average = level_sums[i] as f64 / level_counts[i] as f64;
    //             res.push(average);
    //         }
    //     }
    //     res
    // }

    // pub fn average(t: Rc<RefCell<TreeNode>>, i: usize, sums: &mut Vec<i64>, counts: &mut Vec<i32>) 
    // {
    //     let borrow = t.borrow();

    //     if i < sums.len() 
    //     {
    //         // if level is < sums list size, update value
    //         sums[i] += borrow.val as i64;
    //         counts[i] += 1;
    //     } 
    //     else 
    //     {
    //         // if level is == to sums list size, add new value
    //         sums.push(borrow.val as i64);
    //         counts.push(1);
    //     }

    //     // left.clone() creates a new Rc (type Rc<RefCell<TreeNode>>) for child
    //     if let Some(ref left) = &borrow.left     { Self::average(left.clone(), i+1, sums, counts); }
    //     if let Some(ref right) = &borrow.right   { Self::average(right.clone(), i+1, sums, counts); }
    // }

    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> 
    {
        let Some(root_node) = root else { return Vec::new(); };

        let mut res: Vec<f64> = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root_node);

        while !queue.is_empty() 
        {
            let (mut sum, mut count): (i64, i32) = (0, 0);
            let mut temp: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
            
            // while let Some(n) = queue.pop_front() 
            while !queue.is_empty()
            {
                let n = queue.pop_front().unwrap();
                let borrow = n.borrow();

                sum += borrow.val as i64;
                count += 1;

                if let Some(ref left) = borrow.left     { temp.push_back(left.clone()); }
                if let Some(ref right) = borrow.right   { temp.push_back(right.clone()); }
            }
            queue = temp;
            if count > 0 { res.push(sum as f64 * 1 as f64 / count as f64); }
        }
        res
    }
}