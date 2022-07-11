#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int a[17][17];
bool can[17][17];
int xsize, ysize;
void go()
{
  int i, j;
  for (i = xsize; i >= 0; i--)
  {
    for (j = ysize; j >= 0; j--)
    {
      if (i == xsize && j == ysize)
        a[i][j] = 1;
      else if (can[i][j])
      {
        a[i][j] = a[i + 1][j] + a[i][j + 1];
      }
    }
  }
}
void put(int x, int y)
{
  if (x >= 0 && y >= 0 && x <= xsize && y <= ysize)
  {
    can[x][y] = false;
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  int hx, hy;
  cin >> xsize >> ysize >> hx >> hy;
  memset(a, 0, sizeof(a));
  memset(can, 0, sizeof(can));
  int i, j;
  for (i = 0; i < xsize + 1; i++)
    for (j = 0; j < ysize + 1; j++)
      can[i][j] = true;
  put(hx - 1, hy - 2);
  put(hx - 1, hy + 2);
  put(hx - 2, hy + 1);
  put(hx - 2, hy - 1);
  put(hx + 1, hy + 2);
  put(hx + 1, hy - 2);
  put(hx + 2, hy - 1);
  put(hx + 2, hy + 1);
  put(hx, hy);
  go();
  cout << a[0][0] << endl;
  return 0;
}