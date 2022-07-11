#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int main()
{
  // freopen("in.txt", "r", stdin);
  int n, time;
  int ans = 0;
  cin >> n >> time;
  int i;
  for (i = 0; i < time; i++)
  {
    int temp = n & 1;
    n /= 2;
    ans += n;
    if (temp)
      n++;
  }
  cout << ans << endl;
  return 0;
}