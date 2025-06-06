// Last updated: 6/6/2025, 3:31:45 PM
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
private:
    list<int> output;
    deque<Node*> deq;
public:
    vector<int> preorder(Node* root) {
        if (root == NULL) return vector<int>{make_move_iterator(output.begin()), make_move_iterator(output.end())};

        deq.push_back(root);
        while (!deq.empty()) {
            
            Node* node = deq.front();
            deq.pop_front();
            output.push_back(node->val);
            
            reverse(node->children.begin(), node->children.end());
            for (Node* item : node->children) deq.push_front(item);
        }
        return vector<int>{make_move_iterator(output.begin()), make_move_iterator(output.end())};       
    }
};