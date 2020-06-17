package leetcode;

class P0034 {
static class Solution {
    int lower_bound(int[] nums, int target) {
        int l = 0, r = nums.length;
        while (l < r) {
            int mid = (l + r) >> 1;
            if (nums[mid] < target) {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if (l < nums.length && nums[l] == target) {
            return l;
        } else {
            return -1;
        }
    }

    int upper_bound(int[] nums, int target) {
        int l = 0, r = nums.length;
        while (l + 1 < r) {
            int mid = (l + r) >> 1;
            if (nums[mid] <= target) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        if (r > 0 && r < nums.length && nums[r] == target) {
            return r;
        } else if (nums[l] == target) {
            return l;
        } else {
            return -1;
        }
    }

    public int[] searchRange(int[] nums, int target) {
        if (nums.length == 0) return new int[]{-1, -1};
        int l = lower_bound(nums, target);
        int u = upper_bound(nums, target);
        return new int[]{l, u};
    }

}

public static void main(String[] args) {
    int[] res = new Solution().searchRange(new int[]{1}, 1);
    System.out.println(res[0] + " , " + res[1]);
}
}
