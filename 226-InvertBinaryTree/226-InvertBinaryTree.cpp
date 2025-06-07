// Last updated: 6/7/2025, 2:41:59 AM
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
    // TreeNode* invertTree(TreeNode* root) {
    //     TreeNode *temp = nullptr; 
    //     return root ? (temp = invertTree(root->right), root->right = invertTree(root->left), root->left = temp, root) : root;
    // }

    TreeNode* invertTree(TreeNode* root) {
        if (root == nullptr) return nullptr;

        queue<TreeNode *> queue;
        queue.push(root);
        
        while (!queue.empty()) {
            TreeNode *current = queue.front();
            TreeNode *temp = current->left;
            current->left = current->right;
            current->right = temp;
            if (current->left != nullptr) queue.push(current->left);
            if (current->right != nullptr) queue.push(current->right);
            queue.pop();
        }
        return root;
    }

};