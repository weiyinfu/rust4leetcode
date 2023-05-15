#include <iostream>
#include <stdio.h>
using namespace std;
double a, b, c, d;
double f(double x)
{
  return a * x * x * x + b * x * x + c * x + d;
}
double go(double from, double to)
{
  if (to - from < 0.004)
    return from;
  double m = (from + to) / 2;
  double fm = f(m);
  if (fm * f(from) < 0)
    return go(from, m);
  else
    return go(m, to);
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> a >> b >> c >> d;
  double i, j;
  for (i = -100; i < 100; i++)
  {
    if (f(i) == 0)
      printf("%.2lf ", i);
    else if (f(i) * f(i + 1) < 0)
    {
      printf("%.2lf ", go(i, i + 1));
    }
  }
  return 0;
}