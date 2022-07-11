#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
long long int g[17][23];
int n, a, b;
void init()
{
  int i, j, k;
  memset(g, 0, sizeof(g));
  for (i = 0; i <= n; i++)
    g[0][i] = 1;
  for (i = 1; i <= 15; i++)
  {
    for (j = 1; j <= n; j++)
    {
      g[i][j] = 0;
      for (k = 0; k <= i; k++)
      {
        g[i][j] += g[i - k][j - 1];
      }
    }
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> n >> a >> b;
  init();
  int i;
  unsigned long long int ga, gb;
  ga = gb = 0;
  for (i = 0; i <= a; i++)
  {
    ga += g[a - i][n];
  }
  for (i = 0; i <= b; i++)
    gb += g[b - i][n];
  cout << (unsigned long long)(ga * gb) << endl;
  return 0;
}
