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
    bool isBalanced(TreeNode* root) {
        if (!root) {
            return true;
        }

        int l = depthOfTree(root->left);
        int r = depthOfTree(root->right);
        if (std::abs(l-r) > 1) {
            return false;
        }

        return isBalanced(root->left) && isBalanced(root->right);
    }

    int depthOfTree(TreeNode* root) {
        if (!root) {
            return 0;
        }
        int l = depthOfTree(root->left);
        int r = depthOfTree(root->right);
        return std::max(l, r) + 1;
    }
};
