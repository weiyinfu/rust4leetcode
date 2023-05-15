#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
int a[9][9] = {
    {0, 1, 3, 4, -1},
    {0, 1, 2, -1},
    {1, 2, 4, 5, -1},
    {0, 3, 6, -1},
    {1, 3, 4, 5, 7, -1},
    {2, 5, 8, -1},
    {3, 4, 6, 7, -1},
    {6, 7, 8, -1},
    {4, 5, 7, 8, -1}};
int c[9];
int o[9];
int mi;
int ans[9];
bool ok()
{
  int i;
  for (i = 0; i < 9; i++)
    if (c[i])
      return false;
  return true;
}
void change(int op)
{
  int i;
  for (i = 0; a[op][i] != -1; i++)
  {
    c[a[op][i]]++;
    c[a[op][i]] %= 4;
  }
}
void go(int op, int now)
{
  if (ok())
  {
    mi = now;
    memcpy(ans, o, sizeof(o));
    return;
  }
  if (op == 9 || now >= mi)
    return;
  int i;
  for (i = 0; i < 4; i++, change(op))
  {
    o[op] = i;
    go(op + 1, now + i);
  }
  o[op] = 0;
}
int main()
{
  int i;
  for (i = 0; i < 9; i++)
    cin >> c[i];
  mi = 27;
  go(0, 0);
  for (i = 0; i < 9; i++)
  {
    while (ans[i]--)
      cout << i + 1 << " ";
  }
  return 0;
}