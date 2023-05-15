#include <iostream>
#include <string.h>
using namespace std;
bool road[100003];
int len;
int di;
int main()
{
  cin >> len >> di;
  int i, j;
  int f, t;
  memset(road, -1, sizeof(road));
  for (i = 0; i < di; i++)
  {
    cin >> f >> t;
    for (j = f; j <= t; j++)
      road[j] = false;
  }
  int ans = 0;
  for (i = 0; i <= len; i++)
  {
    if (road[i])
      ans++;
  }
  cout << ans << endl;
  return 0;
}