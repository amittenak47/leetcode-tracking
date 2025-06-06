// Last updated: 6/6/2025, 3:31:45 PM

// Definition for a Node.
// class Node {
// public:
//     int val;
//     vector<Node*> children;

//     Node() {}

//     Node(int _val) {
//         val = _val;
//     }

//     Node(int _val, vector<Node*> _children) {
//         val = _val;
//         children = _children;
//     }
// };


class Solution {
public:
    vector<int> postorder(Node* root) {
        // Iterative
        list<int> output;
	    deque<Node *> stack;

        if (root == NULL) return vector<int>{ make_move_iterator(std::begin(output)), make_move_iterator(std::end(output)) };
        
        stack.push_back(root);
        while (!stack.empty()) {
            Node *cur = stack.back();
            stack.pop_back();
            if (cur != NULL) {
                output.push_front(cur->val);
            }
            
            for (auto node : cur->children) {
                if (node != NULL) stack.push_back(node);              
            }


        }
        return vector<int>{ make_move_iterator(std::begin(output)), make_move_iterator(std::end(output)) };          
    }
};