#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
double a[101][101];
double b[101];
int size;
void change(int m, int n)
{
  int i;
  for (i = 0; i < size; i++)
  {
    swap(a[m][i], a[n][i]);
  }
  swap(b[m], b[n]);
}
void debug()
{
  int i, j;
  for (i = 0; i < size; i++)
  {
    for (j = 0; j < size; j++)
    {
      cout << a[i][j] << " ";
    }
    cout << b[i] << endl;
  }
}
void go()
{
  int i, j, k;
  for (i = 0; i < size; i++)
  {
    if (a[i][i] == 0)
    {
      for (j = i + 1; j < size; j++)
      {
        if (a[j][i])
        {
          change(j, i);
        }
      }
    }
    for (j = i + 1; j < size; j++)
    {
      double t = a[j][i] / a[i][i];
      for (k = i; k < size; k++)
      {
        a[j][k] -= a[i][k] * t;
      }
      b[j] -= b[i] * t;
    }
  }
  for (i = size - 1; i >= 0; i--)
  {
    for (j = i + 1; j < size; j++)
    {
      b[i] -= a[i][j] * b[j];
    }
    b[i] /= a[i][i];
  }
}
int main()
{
  cin >> size;
  int i, j;
  for (i = 0; i < size; i++)
  {
    for (j = 0; j < size; j++)
    {
      cin >> a[i][j];
    }
    cin >> b[i];
  }
  go();
  for (i = 0; i < size; i++)
    cout << (int)(b[i] + 0.5) << " ";
  return 0;
}