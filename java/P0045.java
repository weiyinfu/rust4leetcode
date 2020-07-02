class Solution {
int get(int[]nums,int i){
    int ans=i-1;
    for(int j=i-1;j>=0;j--){
        if(nums[j]+j>=i){
            ans=j;
        }
    }
    return ans;
}
public int jump(int[] nums) {
    int cnt=0;
    int i=nums.length-1;
    while(i>0){
        i=get(nums,i);
        cnt++;
    }
    return cnt;
}
}