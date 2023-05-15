#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
using namespace std;
int a[1010][1010];
int size;
int min(int a, int b)
{
  if (a < b)
    return a;
  else
    return b;
}
int main()
{
  cin >> size;
  int i, j, k;
  for (i = 0; i < size; i++)
  {
    for (j = 0; j <= i; j++)
    {
      cin >> a[i][j];
    }
  }
  int temp[1010];
  for (i = 1; i < size; i++)
  {
    for (j = 0; j <= i; j++)
    {
      temp[j] = min(a[i - 1][(j + i - 1) % i], a[i - 1][j % i]);
      temp[j] += a[i][j];
    }
    while (true)
    {
      bool changed = false;
      for (j = 0; j <= i; j++)
      {
        if (temp[j] > a[i][j] + temp[(j + 1) % (i + 1)])
        {
          changed = true;
          temp[j] = a[i][j] + temp[(j + 1) % (i + 1)];
        }
        if (temp[j] > a[i][j] + temp[(j + i) % (i + 1)])
        {
          changed = true;
          temp[j] = a[i][j] + temp[(j + i) % (i + 1)];
        }
      }
      if (!changed)
        break;
    }
    for (j = 0; j <= i; j++)
    {
      a[i][j] = temp[j];
    }
  }
  cout << a[size - 1][0] << endl;
  return 0;
}