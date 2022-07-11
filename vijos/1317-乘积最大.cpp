#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
int a[43][7][47];
bool visited[43][7];
char str[43];
int numSize;
int cheng;
void mul(int res[], int x[], int y[])
{
  int i, j;
  for (i = 0; i < 47; i++)
  {
    for (j = 0; j < 47; j++)
    {
      res[i + j] += x[i] * y[j];
    }
  }
  for (i = 0; i < 47; i++)
  {
    res[i + 1] += res[i] / 10;
    res[i] %= 10;
  }
}
bool better(int res[], int ans[])
{
  int i;
  for (i = 46; i >= 0; i--)
  {
    if (res[i] > ans[i])
      return true;
    if (res[i] < ans[i])
      return false;
  }
  return false;
}
void init(int from, int ge)
{
  visited[from][ge] = true;
  int i;
  if (ge == 0)
  {
    for (i = 0; i < numSize - from; i++)
    {
      a[from][ge][i] = str[numSize - 1 - i] - '0';
    }
  }
  else
  {
    int best[47];
    int now[47];
    int res[99];
    memset(best, 0, sizeof(best));
    int j;
    for (i = from; i < numSize - ge; i++)
    {
      memset(now, 0, sizeof(now));
      for (j = i; j >= from; j--)
      {
        now[i - j] = str[j] - '0';
      }
      if (!visited[i + 1][ge - 1])
      {
        init(i + 1, ge - 1);
      }
      memset(res, 0, sizeof(res));
      mul(res, now, a[i + 1][ge - 1]);
      if (better(res, best))
      {
        for (j = 0; j < 47; j++)
          best[j] = res[j];
      }
    }
    for (i = 0; i < 47; i++)
      a[from][ge][i] = best[i];
  }
}
int main()
{
  cin >> numSize >> cheng;
  cin >> str;
  memset(a, 0, sizeof(a));
  memset(visited, 0, sizeof(visited));
  init(0, cheng);
  int i;
  int *ans = a[0][cheng];
  for (i = 46; ans[i] == 0; i--)
    ;
  while (i >= 0)
  {
    cout << ans[i--];
  }
  return 0;
}