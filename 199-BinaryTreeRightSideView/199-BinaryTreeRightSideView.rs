// Last updated: 6/7/2025, 2:42:07 AM
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
use std::collections::VecDeque;

impl Solution 
{
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> 
    // {
    //     let Some(root_node) = root else { return Vec::new(); };

    //     let mut right_side: Vec<i32> = Vec::new();

    //     let mut next_level: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    //     let mut curr_level: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    //     next_level.push_back(root_node);

    //     while !next_level.is_empty()
    //     {
    //         curr_level = std::mem::take(&mut next_level); // take() leaves next_level empty without deallocating

    //         let mut curr_front: Option<Rc<RefCell<TreeNode>>> = None;
    //         while let Some(front) = curr_level.pop_front() 
    //         {
    //             curr_front = Some(front.clone());     // .clone() on Rc increments reference count, not a deep copy
    //             let borrow = front.borrow();          // Borrow node to access its fields.

    //             if let Some(left)  = &borrow.left  { next_level.push_back(left.clone());  }
    //             if let Some(right) = &borrow.right { next_level.push_back(right.clone()); }
    //         }
    //         if let Some(right_node) = curr_front { right_side.push(right_node.borrow().val); }
    //     }
        
    //     right_side
    // }

    // {
    //     let Some(root_node) = root else { return Vec::new(); };

    //     let mut right_side: Vec<i32> = Vec::new();

    //     // Some(node) for actual tree nodes, and None as a sentinel
    //     let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        
    //     queue.push_back(Some(root_node));
    //     queue.push_back(None);

    //     let mut sentinel_prior: Option<Rc<RefCell<TreeNode>>> = None;
    //     while let Some(dq_item) = queue.pop_front() 
    //     {
    //         if let Some(curr_node) = dq_item 
    //         {
    //             sentinel_prior = Some(curr_node.clone());
    //             let borrow = curr_node.borrow();

    //             if let Some(left) = &borrow.left { queue.push_back(Some(left.clone())); }
    //             if let Some(right) = &borrow.right { queue.push_back(Some(right.clone())); }
    //         } 
    //         else 
    //         { 
    //             // sentinel_prior (if Some) was rightmost node of curr level
    //             if let Some(right_node) = sentinel_prior.take() { right_side.push(right_node.borrow().val); }
                
    //             // Add new sentinel to mark the end of next level
    //             if !queue.is_empty() { queue.push_back(None); }
    //         }
    //     }
    //     right_side
    // }

    {
        let Some(root_node) = root else { return Vec::new(); };

        let mut right_side: Vec<i32> = Vec::new();
        let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(root_node);

        while !queue.is_empty() 
        {
            // Get the number of nodes at the current level
            let level_size = queue.len();
            
            // Store rightmost node value of current level, in last update when i == level_size - 1
            for i in 0..level_size 
            {
                let curr_front = queue.pop_front().unwrap();
                let borrow = curr_front.borrow();

                // Add value if this is last node of current level 
                if i == level_size - 1 { right_side.push(borrow.val); }

                if let Some(left) = &borrow.left { queue.push_back(left.clone()); }
                if let Some(right) = &borrow.right { queue.push_back(right.clone()); }
            }
        }
        right_side
    }
}

