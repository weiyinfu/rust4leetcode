#include<iostream>
#include<string.h>
using namespace std;       
int ans[3005];
void mul(int k){
	int i;
	for (i = 0; i < 3000; i++){
		ans[i] *= k;
	}
	for (i = 0; i < 3000; i++){ 
		ans[i + 1] += ans[i] / 10;
		ans[i] %= 10;
	}
}
int main(){  
	int n;
	cin >> n;
	if (n == 1){
		cout << 0 << endl;
		return 0;
	}
	memset(ans, 0, sizeof(ans));
	ans[0] = 1;
	int i,j;
	for (i = 3; i <=n; i++){
		mul(i);
		if (i & 1){
			for (j = 0; ans[j] == 0; j++)ans[j] = 9;
			ans[j]--;
		}
		else{
			for (j = 0; ans[j] == 9; j++)ans[j] = 0;;
			ans[j]++;
		}
	}
	for (i = 3000; i >= 0 && ans[i] == 0; i--);
	for (; i >= 0; i--)cout << ans[i];
	return 0;
}