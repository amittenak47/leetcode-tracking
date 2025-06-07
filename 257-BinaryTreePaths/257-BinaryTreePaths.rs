// Last updated: 6/7/2025, 2:41:52 AM
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
impl Solution 
{
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> 
    {
        let mut res = Vec::new();
        Self::dfs(root.as_ref(), &mut res, &mut Vec::new());
        res
    }

    // Option: Some, None
    // &RC: immutable shared Reference Counter (*ptr) on heap
    // RefCell: interor mutability of 'node'
    fn dfs(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<String>, path: &mut Vec<String>) 
    {
        let Some(node) = root.map(|n| n.borrow()) else { return; };

        path.push(node.val.to_string());                                                            // Expand path with node

        if node.left.is_none() && node.right.is_none() { res.push(path.join("->")); }               // Push completed path ("->" separated path) at leaf node, to res
        else 
        {
            Self::dfs(node.left.as_ref(), res, path);
            Self::dfs(node.right.as_ref(), res, path);
        }
        path.pop();                                                                                 // Remove node from explored path before returning
    }
}