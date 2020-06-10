import java.util.Arrays;
import java.util.Map;
import java.util.TreeMap;
import java.util.TreeSet;

public class P0081 {
public class Solution {
    /**
     * @param nums: A list of integers
     * @return: the median of numbers
     */
    public int[] medianII(int[] nums) {
        // write your code here
        TreeMap<Integer, Integer> a = new TreeMap<>();
        int[] ans = new int[nums.length];
        ans[0] = nums[0];
        //now始终指向最后一个元素
        int now = nums[0];
        int which = 0;//which表示当前now的下标
        a.put(nums[0], 1);
        for (int i = 1; i < nums.length; i++) {
            int v = nums[i];
            a.put(v, a.getOrDefault(v, 0) + 1);
            if (v <= now) {
                //which始终表示now的最后一个元素
                which++;
            }
            int should = i / 2;
            if (should > which) {
                //需要往后移动
                Map.Entry<Integer, Integer> next = a.ceilingEntry(now + 1);
                which += next.getValue();
                now = next.getKey();
            } else if (should < which) {
                //需要往前移动
                int nowCount = a.get(now);
                if (which - should >= nowCount) {
                    which -= nowCount;
                    now = a.floorKey(now - 1);
                }
            }
            System.out.println("i=" + i + " which=" + which + " now=" + now + " should=" + should);
            ans[i] = now;
        }
        return ans;
    }
}

public static void main(String[] args) {
    int[] a = {1,2,3,4,5};
    int[] ans = new P0081().new Solution().medianII(a);
    for (int i : ans) {
        System.out.print(i + ",");
    }
}
}
