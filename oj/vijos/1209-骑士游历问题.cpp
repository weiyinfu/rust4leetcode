#include <iostream>
#include <string.h>
#include <math.h>
#include <stdio.h>
using namespace std;
bool huzhi(int m, int n)
{
  int i;
  int mi = m;
  if (mi > n)
    mi = n;
  for (i = 2; i * i <= mi; i++)
  {
    if (m % i == 0 && n % i == 0)
      return false;
  }
  return true;
}
int main()
{
  int size;
  cin >> size;
  int m, n;
  while (size--)
  {
    cin >> m >> n;
    if (huzhi(m, n) && ((m + n) & 1))
      cout << "y";
    else
      cout << 'n';
  }
  return 0;
}