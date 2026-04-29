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
    int goodNodes(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        int result = 1;
        goodNodesHelper(root->left, root->val, result);
        goodNodesHelper(root->right, root->val, result);
        return result;
    }

    void goodNodesHelper(TreeNode* root, int max, int& result) {
        if (root == nullptr) {
            return;
        }
        if (max <= root->val) {
            result++;
            max = root->val;
        }
        goodNodesHelper(root->left, max, result);
        goodNodesHelper(root->right, max, result);
    }
};
