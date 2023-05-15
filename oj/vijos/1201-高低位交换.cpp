#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int main()
{
  // freopen("in.txt", "r", stdin);
  long long int n;
  cin >> n;
  long long int h = n / (1 << 16);
  long long int l = n % (1 << 16);
  long long int ans = (l << 16);
  ans += h;
  cout << ans << endl;
  return 0;
}