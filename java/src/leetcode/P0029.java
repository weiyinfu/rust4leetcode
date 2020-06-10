class Solution {
public int divide(int dividend, int divisor) {
    long a=dividend,b=divisor;
    if(a==Integer.MIN_VALUE&&b==-1)return Integer.MAX_VALUE;
    if(b==0)return Integer.MAX_VALUE;
    int signal=(a<0?-1:1)*(b<0?-1:1);
    a=a<0?-a:a;
    b=b<0?-b:b;

    if(b>a)return 0;
    if(b==a)return signal;

    int i=0;
    while(b<a){b<<=1;i++;}
    int ans=0;
    while(i-->=0){
        //System.out.println(ans+" "+a+" "+b);
        if(b<=a){
            ans=ans<<1|1;
            a-=b;
            b>>=1;
        }else{
            ans<<=1;
            b>>=1;
        }
    }
    return ans*signal;
}
}