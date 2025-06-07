// Last updated: 6/7/2025, 2:41:48 AM
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
private:
    TreeNode* previous;
    TreeNode* inorderSuccessorNode;

    void inorderCase1(TreeNode* p) {
        TreeNode* leftmost = p->right;
        while (leftmost->left != NULL) leftmost = leftmost->left;
        inorderSuccessorNode = leftmost;
    }

    void inorderCase2(TreeNode* node, TreeNode* p) {
        
        if (node == NULL) return;
        
        inorderCase2(node->left, p); // Recurse on the left side
        
        if (previous == p && inorderSuccessorNode == NULL) { // Check if previous node inorder-precedes node p
            inorderSuccessorNode = node;
            return;
        }
        previous = node; // update previous node
        
        inorderCase2(node->right, p); // Recurse on the right side
    }
public:
    TreeNode* inorderSuccessor(TreeNode* root, TreeNode* p) {
        
        // if (p->right != NULL) inorderCase1(p); // inorder successor is min node > p which is left-most of p->right    
        // else inorderCase2(root, p); // inorder traversal (L-P-R) check if previous = p, then current is successor
        
        // return inorderSuccessorNode;
        
        TreeNode* successor = NULL;
        while (root != NULL) {
            
            if (p->val >= root->val) root = root->right; // use BST, traverse right; if p = root
            else {
                successor = root;
                root = root->left;
            }
        }
        
        return successor;
    }
};