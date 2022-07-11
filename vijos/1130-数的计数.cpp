#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int f(int n)
{
  if (n == 1)
    return 1;
  int ans = 1;
  int i;
  for (i = 1; i * 2 <= n; i++)
  {
    ans += f(i);
  }
  return ans;
}
int main()
{
  // freopen("in.txt", "r", stdin);
  int n;
  cin >> n;
  cout << f(n) << endl;
  return 0;
}