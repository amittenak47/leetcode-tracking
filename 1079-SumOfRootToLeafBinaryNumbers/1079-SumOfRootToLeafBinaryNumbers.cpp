// Last updated: 6/6/2025, 3:31:35 PM
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:    
    int rootToLeaf = 0;
    
    int sumRootToLeaf(TreeNode *root) {
        preorder(root, 0);
        return rootToLeaf;
    }
    
    void preorder(TreeNode *r, int currNumber) {
        if (r != nullptr) {
            currNumber = (currNumber << 1) | r->val;
            if (r->left == nullptr && r->right == nullptr) { rootToLeaf += currNumber; }
            
            preorder(r->left, currNumber); 
            preorder(r->right, currNumber);
        }
    }


};