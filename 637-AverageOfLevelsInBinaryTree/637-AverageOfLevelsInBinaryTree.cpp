// Last updated: 6/6/2025, 3:31:55 PM
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
    vector<double> averageOfLevels(TreeNode* root) {
        vector<double> res;
        std::queue<TreeNode*> queue;
        queue.push(root);

        while (!queue.empty()) {
            long sum = 0, count = 0;
            std::queue<TreeNode*> temp;

            while (!queue.empty()) {
                TreeNode *n = queue.front();
                queue.pop();

                sum += n->val;
                count++;
                
                if (n->left != nullptr) { temp.push(n->left); }
                if (n->right != nullptr) { temp.push(n->right); }
            }

            queue = temp;
            res.push_back(sum * 1.0 / count);
        }
        return res;
    }
};