// Last updated: 6/6/2025, 3:31:57 PM
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
    int diameter = 0; 
    int diameterOfBinaryTree(TreeNode* root) {
       longestPath(root);
       return diameter;
    }

    int longestPath(TreeNode *root) {
        if(root == nullptr) return 0; // base case: parent leaf node, return 0 

        int leftPath = longestPath(root->left);
        int rightPath = longestPath(root->right);
        diameter = max(diameter, leftPath + rightPath); // update diameter with max of left+right paths

        return max(leftPath, rightPath) + 1; // increment path by 1 to include current node
    }
};