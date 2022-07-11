#include <iostream>
#include <string.h>
#include <math.h>
#include <stdio.h>
using namespace std;
char a[10004];
int n[10004];
int n1[10004];
int len;
bool mod4(int m[])
{
  if (m[0] % 2)
    return false;
  int i;
  int dif = 0;
  for (i = len + 2; i >= 0; i--)
  {
    dif *= 10;
    dif += m[i];
    dif %= 4;
  }
  return dif == 0;
}
int main()
{
  cin >> a;
  memset(n, 0, sizeof(n));
  memset(n1, 0, sizeof(n1));
  int i = 0;
  while (a[i])
    i++;
  len = i;
  for (i = 0; i < len; i++)
  {
    n[i] = n1[i] = a[len - 1 - i] - '0';
  }
  i = 0;
  while (n1[i] == '9')
    i++;
  n1[i]++;
  if (mod4(n))
  {
    cout << 0 << endl;
  }
  else if (mod4(n1))
  {
    cout << 0 << endl;
  }
  else if (n[0] % 2 == 0 && n[1] % 2 == 0)
  {
    cout << 0 << endl;
  }
  else
    cout << 1 << endl;
  return 0;
}