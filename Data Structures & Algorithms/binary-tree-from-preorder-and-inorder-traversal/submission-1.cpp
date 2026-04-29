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
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        //std::cout << "preorder = " << preorder.size() << ", inorder = " << inorder.size() << std::endl;
        if (preorder.empty()) {
            return nullptr;
        }
        TreeNode* root = new TreeNode(preorder[0]);
        auto pos = std::find(inorder.begin(), inorder.end(), preorder[0]) - inorder.begin();
        //std::cout << "inorder pos = " << pos << " for " << preorder[0] << std::endl;
        
        if (pos != 0) {
            vector<int> preorder_left(std::next(preorder.begin()), std::next(preorder.begin() + pos));
            vector<int> preorder_right(std::next(preorder.begin() + pos), preorder.end());
            vector<int> inorder_left(inorder.begin(), inorder.begin() + pos);
            vector<int> inorder_right(std::next(inorder.begin() + pos), inorder.end());

            /*std::cout << "preorder_left = " << preorder_left.size() 
                      << ", preorder_right = " << preorder_right.size()
                      << ", inorder_left = " << inorder_left.size() 
                      << ", inorder_right = " << inorder_right.size() << std::endl;*/
            root->left  = buildTree(preorder_left, inorder_left);
            root->right = buildTree(preorder_right, inorder_right);
        } else {
            vector<int> preorder_right(std::next(preorder.begin()), preorder.end());
            vector<int> inorder_right(std::next(inorder.begin() + pos), inorder.end());

            /*std::cout << "preorder_right = " << preorder_right.size()
                      << ", inorder_right = " << inorder_right.size() << std::endl;*/
            root->right = buildTree(preorder_right, inorder_right);
        }
        return root;    
    }
};
