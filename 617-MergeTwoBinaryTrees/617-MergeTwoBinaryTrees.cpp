// Last updated: 6/6/2025, 3:31:52 PM
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
    TreeNode* mergeTrees(TreeNode* t1, TreeNode* t2) {
        
        // Recursive
        
        // if (t1 == nullptr) return t2;
        // if (t2 == nullptr) return t1;
        
        // t1->val += t2->val;
        // t1->left = mergeTrees(t1->left, t2->left);
        // t1->right = mergeTrees(t1->right, t2->right);
        
        // return t1;

        // Iteration 
        if (t1 == nullptr) return t2;

        deque<vector<TreeNode*>> stack;
        stack.push_back({t1, t2});

        while (!stack.empty()) {
            
            vector<TreeNode*> t = stack.back();
            stack.pop_back();

            if (t[0] == nullptr || t[1] == nullptr) continue;
            
            t[0]->val += t[1]->val;
            if (t[0]->left == nullptr) t[0]->left = t[1]->left;
            else stack.push_back({t[0]->left, t[1]->left});
            
            if (t[0]->right == nullptr) t[0]->right = t[1]->right;
            else stack.push_back({t[0]->right, t[1]->right});
        }
        return t1;
    }
};