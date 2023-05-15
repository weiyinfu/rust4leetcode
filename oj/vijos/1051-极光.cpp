#include <iostream>
#include <string.h>
#include <stdio.h>
#include <math.h>
using namespace std;
bool a[105][105];
int xsize, ysize;
int ans = 0;
int ab(int n)
{
  if (n < 0)
    return -n;
  else
    return n;
}
void visit(int x, int y)
{
  if (x < 0 || y < 0)
    return;
  if (x >= xsize || y > ysize)
    return;
  if (a[x][y] == false)
    return;
  a[x][y] = false;
  visit(x - 2, y);
  int i, j;
  for (i = -2; i <= 2; i++)
  {
    for (j = -(ab(2 - ab(i))); j <= ab(2 - ab(i)); j++)
    {
      visit(x + i, y + j);
    }
  }
}
void go()
{
  int i, j;
  for (i = 0; i < xsize; i++)
  {
    for (j = 0; j < ysize; j++)
    {
      if (a[i][j])
      {
        ans++;
        visit(i, j);
      }
    }
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> xsize >> ysize;
  int i, j;
  char c;
  memset(a, 0, sizeof(a));
  for (i = 0; i < xsize; i++)
    for (j = 0; j < ysize; j++)
    {
      cin >> c;
      if (c == '#')
        a[i][j] = true;
    }
  go();
  cout << ans << endl;
  return 0;
}