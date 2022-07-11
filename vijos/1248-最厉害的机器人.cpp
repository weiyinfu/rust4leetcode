#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int main()
{
  // freopen("in.txt", "r", stdin);
  double x1, x2, x3, x4, k;
  cin >> k >> x1 >> x2 >> x3 >> x4;
  int i, j, p, q;
  double mm = 100000;
  double now;
  for (i = 0; i <= k && i < 11; i++)
  {
    for (j = 0; j <= k - i && j < 11; j++)
    {
      for (p = 0; p <= k - i - j && p < 11; p++)
      {
        for (q = 0; q <= k - i - j - p && q < 11; q++)
        {
          now = (20 - i * 2) * x1;
          double temp = (10 + q) * x4;
          double tt = (100 - j * 7) * x2 * 2 + (100 - p * 9) * x3;
          now += tt / temp;
          if (now < mm)
            mm = now;
        }
      }
    }
  }
  printf("%.3lf", mm);
  return 0;
}
