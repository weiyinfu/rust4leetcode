#include <iostream>
#include <string.h>
#include <math.h>
#include <stdio.h>
using namespace std;
int a[505][505];
int ans[505][505];
int xsize, ysize;
int f(int x, int y)
{
  if (x < 0 || y < 0)
    return 0;
  if (x >= xsize || y >= ysize)
    return 0;
  if (ans[x][y] == -1)
  {
    ans[x][y] = 0;
    if (a[x][y] > a[x - 1][y])
      ans[x][y] = f(x - 1, y);
    if (a[x][y] > a[x + 1][y] && ans[x][y] < f(x + 1, y))
      ans[x][y] = f(x + 1, y);
    if (a[x][y] > a[x][y - 1] && ans[x][y] < f(x, y - 1))
      ans[x][y] = f(x, y - 1);
    if (a[x][y] > a[x][y + 1] && ans[x][y] < f(x, y + 1))
      ans[x][y] = f(x, y + 1);
    ans[x][y]++;
  }
  return ans[x][y];
}
int main()
{
  cin >> xsize >> ysize;
  memset(ans, -1, sizeof(ans));
  int i, j;
  for (i = 0; i < xsize; i++)
  {
    for (j = 0; j < ysize; j++)
    {
      cin >> a[i][j];
    }
  }
  int ma = 0;
  for (i = 0; i < xsize; i++)
  {
    for (j = 0; j < ysize; j++)
    {
      if (ma < f(i, j))
        ma = f(i, j);
    }
  }
  cout << ma << endl;
  return 0;
}