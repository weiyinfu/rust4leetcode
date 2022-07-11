#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
int g[203][203];
int f(int n, int k)
{
  if (n < k)
    return 0;
  if (g[n][k] == -1)
  {
    if (k == 1)
      return 1;
    g[n][k] = f(n - 1, k - 1) + f(n - k, k);
  }
  return g[n][k];
}
int main()
{
  int n, k;
  memset(g, -1, sizeof(g));
  cin >> n >> k;
  cout << f(n, k) << endl;
  return 0;
}