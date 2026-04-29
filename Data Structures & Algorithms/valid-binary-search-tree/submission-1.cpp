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
    bool isValidBST(TreeNode* root) {
        TreeNode* prev = nullptr;
        return isValidBSTHelper(prev, root);
    }

    bool isValidBSTHelper(TreeNode*& prev, TreeNode* root) {
        if (root == nullptr) {
            return true;
        }
        if (!isValidBSTHelper(prev, root->left)) {
            return false;
        }
        if (prev != nullptr && prev->val >= root->val) {
            return false;
        }
        prev = root;
        if (!isValidBSTHelper(prev, root->right)) {
            return false;
        }
        return true;
    }
};
