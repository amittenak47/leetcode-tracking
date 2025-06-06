// Last updated: 6/6/2025, 3:31:54 PM
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
    bool isSubtree(TreeNode* root, TreeNode* subRoot) {

        return root // Check if this node is Empty, then no tree is rooted at this Node
            ? (isIdentical(root, subRoot)) // Check if the "tree rooted at root" is identical to "tree roooted at subRoot"
                ? true // return true
                : isSubtree(root->left, subRoot) || isSubtree(root->right, subRoot) // Else, Check for whether the "tree rooted at root.left" and "tree rooted at root.right" return true
            : false; // return false
        
    }

    bool isIdentical(TreeNode* node1, TreeNode* node2) {

        return (node1 && node2) // If both nodes are non-empty they are identical only if
            ? (node1->val == node2->val) // they are equal
                && (isIdentical(node1->left, node2->left)) // left subtrees are identical
                && (isIdentical(node1->right, node2->right)) // right subtrees are identical
            : (node1 == nullptr && node2 == nullptr); // If any of the nodes is null, then both must be null
    }
};