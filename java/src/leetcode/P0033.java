import java.util.Arrays;

class Solution {
public int search(int[] nums, int target) {
    if(nums.length==0){
        return -1;
    }
    int l = 0, r = nums.length;
    while (l < r) {
        int mid = (l + r) >> 1;
        if (nums[mid] >= nums[0]) {
            l = mid + 1;
        } else {
            r = mid;
        }
    }
    System.out.println("min index "+r);
    int minInd = r;
    int pos = -1;
    if(minInd<nums.length&&nums[minInd]==target)return minInd;
    if (target >= nums[0]) {
        pos = Arrays.binarySearch(nums, 0, minInd, target);
    } else {
        pos = Arrays.binarySearch(nums, minInd, nums.length, target);
    }
    if(pos<0||pos>=nums.length)pos=-1;
    return pos;
}

public static void main(String[] args) {
    int ans = new Solution().search(new int[]{1,1}, 1);
    System.out.println(ans);
}
}