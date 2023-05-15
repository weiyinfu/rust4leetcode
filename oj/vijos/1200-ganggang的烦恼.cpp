#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
int a[3000];
void mul(int k)
{
  int i;
  for (i = 0; i < 2997; i++)
  {
    a[i] *= k;
  }
  for (i = 0; i < 2997; i++)
  {
    a[i + 1] += a[i] / 10;
    a[i] %= 10;
  }
}
int main()
{
  int n;
  cin >> n;
  memset(a, 0, sizeof(a));
  if (n == 0 || n == 1)
  {
    cout << 1 << 'F' << endl;
    return 0;
  }
  a[0] = 1;
  int i;
  for (i = 2; i <= n; i++)
  {
    mul(i);
  }
  int y = 0;
  for (i = 0; i < 3000; i++)
  {
    y += a[i];
  }
  for (i = 2; i < y; i++)
  {
    if (y % i == 0)
    {
      break;
    }
  }
  if (y == i)
  {
    cout << y << 'T' << endl;
  }
  else
  {
    cout << y << "F" << endl;
  }
  return 0;
}