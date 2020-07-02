import java.lang.Math.*;
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
class Pair{
    boolean is=false;
    int depth;
    Pair(boolean is,int depth){
        this.is=is;
        this.depth=depth;
    }
}
Pair go(TreeNode root){
    if(root==null){
        return new Pair(true,0);
    }
    Pair left=go(root.left);
    Pair right=go(root.right);
    boolean is=left.is&&right.is&&(left.depth+1==right.depth||left.depth-1==right.depth||left.depth==right.depth);
    return new Pair(is,Math.max(left.depth,right.depth)+1) ;
}
public boolean isBalanced(TreeNode root) {
    return go(root).is;
}
}