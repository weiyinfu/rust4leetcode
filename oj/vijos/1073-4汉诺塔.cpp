#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
int haha[50005];
void init()
{
  int i;
  haha[0] = 1;
  for (i = 1; i < 50005; i++)
  {
    haha[i] = (haha[i - 1] << 1);
    haha[i] %= 10000;
  }
}
int main()
{
  int n;
  init();
  cin >> n;
  int ans = 0;
  int i = 0, j = 0, k;
  while (i < n)
  {
    j++;
    for (k = 0; k < j && i < n; k++, i++)
    {
      ans += haha[j - 1];
      ans %= 10000;
    }
  }
  cout << ans << endl;
  return 0;
}
