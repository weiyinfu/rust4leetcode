#include <iostream>
#include <string.h>
using namespace std;
bool zhi(int m, int n)
{
  int i;
  int k = m;
  if (k > n)
    k = n;
  for (i = 2; i <= k; i++)
  {
    if (m % i == 0 && n % i == 0)
      return false;
  }
  return true;
}
int main()
{
  // freopen("in.txt", "r", stdin);
  int a, b, l;
  cin >> a >> b >> l;
  double k = a;
  k /= b;
  int i, j;
  int mi = l, mj = 1;
  double mm = l;
  for (i = 1; i <= l; i++)
  {
    for (j = 1; j <= l; j++)
    {
      if (zhi(i, j))
      {
        double temp = i;
        temp /= j;
        if (temp >= k)
        {
          if (temp < mm)
          {
            mm = temp;
            mi = i;
            mj = j;
          }
        }
      }
    }
  }
  cout << mi << " " << mj << endl;
  return 0;
}
