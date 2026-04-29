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
    int kthSmallest(TreeNode* root, int k) {
        return kthSmallestHelper(root, k);
    }

    int kthSmallestHelper(TreeNode* root, int& k) {
        if (root == nullptr) {
            return -1;
        }
        
        if (int ret = kthSmallestHelper(root->left, k); ret >= 0) {
            return ret;
        }

        if (--k == 0) {
            return root->val;
        }
        
        if (int ret = kthSmallestHelper(root->right, k); ret >= 0) {
            return ret;
        }
        return -1;
    }
};
