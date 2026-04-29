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
    int diameterOfBinaryTree(TreeNode* root) {
        int global = 0;
        Solution::depthOfBinaryTree(root, global);
        return global;
    }

    int depthOfBinaryTree(TreeNode* root, int& global) {
        if (!root) {
            return 0;
        }
        int l = Solution::depthOfBinaryTree(root->left, global);
        int r = Solution::depthOfBinaryTree(root->right, global);
        int local = std::max(l, r)+1;
        global = std::max(global, l+r);
        return local;
    }
};
