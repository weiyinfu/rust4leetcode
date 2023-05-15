#include <iostream>
#include <string.h>
#include <math.h>
#include <stdio.h>
using namespace std;
int main()
{
  // freopen("in.txt", "r", stdin);
  int a, b, c, d;
  cin >> a >> b >> c >> d;
  int x = c - a;
  int y = sqrt(b) + sqrt(d);
  double ans = sqrt(x * x + y * y);
  printf("%.3lf\n", ans);
  return 0;
}