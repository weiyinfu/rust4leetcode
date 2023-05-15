#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
long long int f(int m, int n)
{
  if (m == 0 || n == 0)
    return 0;
  if (m == n)
    return m;
  if (m > n)
  {
    return (m / n) * n + f(m % n, n);
  }
  else
  {
    return (n / m) * m + f(n % m, m);
  }
}
int main()
{
  long long int m, n;
  int i;
  for (i = 0; i < 10; i++)
  {
    cin >> m >> n;
    cout << f(m, n) << endl;
  }
  return 0;
}