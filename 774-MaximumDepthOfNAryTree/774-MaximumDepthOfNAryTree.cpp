// Last updated: 6/6/2025, 3:31:46 PM
/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> children;

    Node() {}

    Node(int _val) {
        val = _val;
    }

    Node(int _val, vector<Node*> _children) {
        val = _val;
        children = _children;
    }
};
*/

class Solution {
public:
    // int maxDepth(Node* root) {

    //   if (root == NULL) { return 0; } 
    //   else if (root->children.empty()) { return 1; } 
    //   else {
        
    //     vector<int> heights;
    //     for (Node *item : root->children) { heights.push_back(maxDepth(item)); }

    //     return *max_element(heights.begin(), heights.end()) + 1;
    //   }
    // }     

  int maxDepth(Node* root) {
    queue<pair<Node*, int>> stack;
    if (root != NULL) { stack.push(pair(root, 1)); }

    int depth = 0;
    while (!stack.empty()) {
      
      pair<Node*, int> current = stack.front();
      stack.pop();
      root = get<0>(current);
      int current_depth = get<1>(current);
      
      if (root != NULL) {
        depth = max(depth, current_depth);
        for (Node* c : root->children) { stack.push(pair(c, current_depth + 1)); }
      }
    }
    return depth;
  }   
};