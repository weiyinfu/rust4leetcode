#include <iostream>
#include <string.h>
using namespace std;
int a[1003][1003];
int main()
{
  // freopen("in.txt", "r", stdin);
  int w, h;
  cin >> w >> h;
  int i, j;
  memset(a, 0, sizeof(a));
  for (i = 0; i < w; i++)
    for (j = 0; j < h; j++)
      cin >> a[i][j];
  int mm = 0;
  for (i = w - 1; i >= 0; i--)
  {
    for (j = h - 1; j >= 0; j--)
    {
      if (a[i][j] == 1)
      {
        int x = a[i][j + 1];
        if (a[i + 1][j] < x)
        {
          x = a[i + 1][j];
        }
        if (a[i + x][j + x] != 0)
          a[i][j] = x + 1;
        else
          a[i][j] = x;
        if (a[i][j] > mm)
          mm = a[i][j];
      }
    }
  }
  cout << mm << endl;
  return 0;
}