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
    vector<int> rightSideView(TreeNode* root) {
        queue<pair<int, TreeNode*>> q;
        if (root != nullptr) {
            q.push({0, root});
        }
        vector<int> result;
        int last = -1;
        while (!q.empty()) {
            auto [level, root] = q.front(); q.pop();
            if (last != level) {
                last = level;
                result.push_back(root->val);
            }
            if (root->right != nullptr) {
                q.push({level+1, root->right});
            }
            if (root->left != nullptr) {
                q.push({level+1, root->left});
            }
        }
        return result;
    }
};
