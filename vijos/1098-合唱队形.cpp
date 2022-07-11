#include <iostream>
#include <string.h>
using namespace std;
int a[103];
int size;
int up[103][103];
int down[103][103];
int u[103][103];
int d[103][103];
void initup()
{
  int i, j, k;
  for (i = 0; i < size; i++)
  {
    up[i][i] = 1;
    for (j = i + 1; j < size; j++)
    {
      up[i][j] = 0;
      for (k = i; k < j; k++)
      {
        if (a[k] < a[j])
        {
          if (up[i][k] > up[i][j])
          {
            up[i][j] = up[i][k];
          }
        }
      }
      up[i][j]++;
    }
  }
}
void initdown()
{
  int i, j, k;
  for (i = 0; i < size; i++)
  {
    down[i][i] = 1;
    for (j = i + 1; j < size; j++)
    {
      down[i][j] = 0;
      for (k = i; k < j; k++)
      {
        if (a[k] > a[j])
        {
          if (down[i][k] > down[i][j])
          {
            down[i][j] = down[i][k];
          }
        }
      }
      down[i][j]++;
    }
  }
}
void init()
{
  int i, j, k;
  for (i = 0; i < size; i++)
  {
    for (j = 0; j < size; j++)
    {
      u[i][j] = up[i][j];
      d[i][j] = down[i][j];
      for (k = i; k < j; k++)
      {
        if (up[i][k] > u[i][j])
          u[i][j] = up[i][k];
        if (down[i][k] > d[i][j])
          d[i][j] = down[i][k];
      }
    }
  }
}
int main()
{
  cin >> size;
  int i;
  for (i = 0; i < size; i++)
  {
    cin >> a[i];
  }
  memset(up, -1, sizeof(up));
  memset(down, -1, sizeof(down));
  initup();
  initdown();
  init();
  int stay = d[0][size - 1];
  if (stay < u[0][size - 1])
    stay = u[0][size - 1];
  for (i = 1; i < size - 1; i++)
  {
    int now = u[0][i - 1] + d[i + 1][size - 1];
    if (stay < now)
      stay = now;
  }
  cout << size - stay << endl;
  return 0;
}