#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
int ans = 0;
int h, l;
int size;
char a[66600];
int go(int from, int to)
{
  int kind;
  if (from == to)
  {
    if (a[from] == '0')
      kind = 5;
    else
      kind = 2;
  }
  else
  {
    int mid = ((to + from) >> 1);
    int left = go(from, mid);
    int right = go(mid + 1, to);
    if (left == 5 && right == 5)
      kind = 5;
    else if (left == 2 && right == 2)
      kind = 2;
    else
      kind = 1;
  }
  int len = to - from + 1;
  int ceng = log2(len);
  // ceng = size - ceng;
  if (ceng <= h && ceng >= l)
    ans += kind;
  return kind;
}
int main()
{
  cin >> size;
  cin >> h >> l;
  cin >> a;
  go(0, (1 << size) - 1);
  cout << ans << endl;
  return 0;
}