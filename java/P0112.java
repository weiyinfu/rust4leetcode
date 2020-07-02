/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode(int x) { val = x; }
 * }
 */
class Solution {
boolean go(TreeNode root,int sum){
    if(root==null){
        return false;
    }
    if(root.left==null &&root.right==null){
        return sum==root.val;
    }
    return go(root.left,sum-root.val)||go(root.right,sum-root.val);
}
public boolean hasPathSum(TreeNode root, int sum) {
    if(root==null)return false;
    return go(root,sum);
}
}