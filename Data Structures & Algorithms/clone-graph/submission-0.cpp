/*
// Definition for a Node.
class Node {
public:
    int val;
    vector<Node*> neighbors;
    Node() {
        val = 0;
        neighbors = vector<Node*>();
    }
    Node(int _val) {
        val = _val;
        neighbors = vector<Node*>();
    }
    Node(int _val, vector<Node*> _neighbors) {
        val = _val;
        neighbors = _neighbors;
    }
};
*/

class Solution {
public:
    Node* cloneGraph(Node* node) {
        if (node == nullptr) {
            return nullptr;
        }
        unordered_map<int, Node*> visited;
        return dfs(node, visited);
    }

    Node* dfs(const Node* node, unordered_map<int, Node*>& visited) {
        Node* cloned = new Node(node->val);
        visited.insert({node->val, cloned});
        for (const auto& neighbor : node->neighbors) {
            if (!visited.contains(neighbor->val)) {
                cloned->neighbors.push_back(dfs(neighbor, visited));
            } else {
                cloned->neighbors.push_back(visited[neighbor->val]);
            }
        }
        return cloned;
    }
};
