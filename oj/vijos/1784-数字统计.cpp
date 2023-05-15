#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int f(int n)
{
  int ans = 0;
  int temp;
  while (n != 0)
  {
    temp = n % 10;
    n /= 10;
    if (temp == 2)
      ans++;
  }
  return ans;
}
int main()
{
  // freopen("in.txt", "r", stdin);
  int from, to;
  cin >> from >> to;
  int i;
  int ans = 0;
  for (i = from; i <= to; i++)
  {
    ans += f(i);
  }
  cout << ans << endl;
  return 0;
}