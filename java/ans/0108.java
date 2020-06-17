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
TreeNode to(int[]nums,int beg,int end){
    if(beg>end)return null;
    int mid=(beg+end)/2;
    TreeNode ans=new TreeNode(nums[mid]);
    ans.left=to(nums,beg,mid-1);
    ans.right=to(nums,mid+1,end);
    return ans;
}
public TreeNode sortedArrayToBST(int[] nums) {
    return to(nums,0,nums.length-1);
}
}