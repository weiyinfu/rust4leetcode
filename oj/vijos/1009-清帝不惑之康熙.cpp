#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
using namespace std;
void gcd(long long int a, long long int b, long long int &d, long long int &x, long long int &y)
{
  if (b == 0)
  {
    d = a;
    x = 1;
    y = 0;
  }
  else
  {
    gcd(b, a % b, d, y, x);
    y -= x * (a / b);
  }
}

int main()
{
  long long int x1, x2, v1, v2, len;
  cin >> x1 >> x2 >> v1 >> v2 >> len;
  int dx, dv;
  v1 %= len;
  v2 %= len;
  x1 %= len;
  x2 %= len;
  if (v1 < v2)
  {
    swap(v1, v2);
    swap(x1, x2);
  }
  dv = v1 - v2;
  if (x1 < x2)
  {
    dx = x2 - x1;
  }
  else
  {
    dx = x2 + len - x1;
  }
  long long int d, k, p;
  gcd(dv, len, d, k, p);
  if (dx % d)
  {
    cout << "Impossible" << endl;
  }
  else
  {
    dv /= d;
    len /= d;
    if (k > 0)
    {
      cout << (dx * k / d) % len << endl;
    }
    else
    {
      cout << ((len + k) * dx / d) % len << endl;
    }
  }
  return 0;
}