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
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> results;
        queue<pair<int, TreeNode*>> q;
        if (root) {
            q.push({0,root});
        }
        int lastLevel = -1;
        while (!q.empty()) {
            auto [level, root] = q.front(); q.pop();
            if (lastLevel != level) {
                lastLevel = level;
                results.push_back({});
            }
            results[level].push_back(root->val);
            if (root->left) {
                q.push({level+1,root->left});
            }
            if (root->right) {
                q.push({level+1,root->right});
            }
        }
        return results;
    }
};
