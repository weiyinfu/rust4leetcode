class Solution {
public boolean all9(int[]digits){
    for(int i:digits){
        if (i!=9){
            return false;
        }
    }
    return true;
}
public int[] plusOne(int[] digits) {
    if (all9(digits)){
        int[]ans=new int[digits.length+1];
        ans[0]=1;
        for (int i=1;i<ans.length;i++){
            ans[i]=0;
        }
        return ans;
    }
    for(int i=digits.length-1;i>=0;i--){
        if(digits[i]==9){
            digits[i]=0;
        }else{
            digits[i]+=1;
            break;
        }
    }
    return digits;
}
}