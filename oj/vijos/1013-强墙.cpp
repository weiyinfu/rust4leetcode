#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
using namespace std;
int size;
struct Wall
{
  double x;
  double y[4];
};
Wall a[25];
double g[25][4][25][4];
double dis(int fw, int fy, int tw, int ty)
{
  double x = a[fw].x - a[tw].x;
  double y = a[fw].y[fy] - a[tw].y[ty];
  return hypot(x, y);
}
bool direct(int fw, int fy, int tw, int ty)
{
  int i;
  double k = (a[fw].y[fy] - a[tw].y[ty]) / (a[fw].x - a[tw].x);
  double b = a[fw].y[fy] - k * a[fw].x;
  for (i = fw + 1; i < tw; i++)
  {
    double y = k * a[i].x + b;
    if (y < a[i].y[0])
      return false;
    if (y < a[i].y[2] && y > a[i].y[1])
      return false;
    if (y > a[i].y[3])
      return false;
  }
  return true;
}
double go(int fw, int fy, int tw, int ty)
{
  if (g[fw][fy][tw][ty] == 0)
  {
    double d = 10000;
    if (direct(fw, fy, tw, ty))
      d = dis(fw, fy, tw, ty);
    else
    {
      int i, j;
      for (i = fw + 1; i < tw; i++)
      {
        for (j = 0; j < 4; j++)
        {
          if (direct(fw, fy, i, j))
          {
            double back = go(i, j, tw, ty);
            double fore = dis(fw, fy, i, j);
            if (fore + back < d)
              d = fore + back;
          }
        }
      }
    }
    g[fw][fy][tw][ty] = d;
  }
  return g[fw][fy][tw][ty];
}
int main()
{
  cin >> size;
  int i, j;
  for (i = 1; i <= size; i++)
  {
    cin >> a[i].x;
    for (j = 0; j < 4; j++)
      cin >> a[i].y[j];
  }
  a[0].x = 0;
  a[size + 1].x = 10;
  for (i = 0; i < 4; i++)
    a[0].y[i] = a[size + 1].y[i] = 5;
  memset(g, 0, sizeof(g));
  printf("%.2f\n", go(0, 0, size + 1, 0));
  return 0;
}