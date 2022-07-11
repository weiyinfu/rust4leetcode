#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
int f(int d, int n)
{
  if (n == 0)
    return 1;
  int t = (n >> 1);
  int p = f(d, t);
  p *= p;
  if (n & 1)
    p *= d;
  p %= 100;
  return p;
}
int main()
{
  int n;
  while (cin >> n && n != 0)
  {
    int four = f(4, n - 1);
    int two = f(2, n - 1);
    four += two;
    if (four % 100 == 0)
      cout << "00" << endl;
    else
      cout << four % 100 << endl;
  }
  return 0;
}