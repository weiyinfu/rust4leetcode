#include <iostream>
#include <string>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <stdio.h>
using namespace std;
bool map[1002][1002];
int line, column;
int ans = 0;
bool go()
{
  int x, y, i, j;
  for (x = 1; x <= line; x++)
  {
    for (y = 1; y <= column; y++)
    {
      if (map[x][y])
      {
        ans++;
        int ymax;
        for (j = y; map[x][j]; j++)
          ;
        ymax = j;
        for (i = x; map[i][y]; i++)
        {
          if (map[i][y - 1])
            return false;
          for (j = y; map[i][j]; j++)
          {
            if (!map[i][j])
              break;
            map[i][j] = false;
          }
          if (j != ymax)
            return false;
        }
      }
    }
  }
}
int main()
{
  cin >> line >> column;
  char c;
  int i, j;
  memset(map, 0, sizeof(map));
  for (i = 1; i <= line; i++)
  {
    for (j = 1; j <= column; j++)
    {
      cin >> c;
      if (c == '#')
        map[i][j] = true;
    }
  }
  if (go())
    printf("There are %d ships.\n", ans);
  else
    cout << "Bad placement." << endl;
  return 0;
}