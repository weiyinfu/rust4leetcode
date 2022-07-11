#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
int a[10003];
int size;
int div(int k)
{
  int i;
  int d = 0;
  for (i = size; i >= 0; i--)
  {
    d *= 10;
    d += a[i];
    d %= k;
  }
  return d;
}
int main()
{
  char s[10003];
  cin >> s;
  int i;
  for (i = 0; s[i]; i++)
    ;
  size = i;
  for (i--; i >= 0; i--)
    a[size - i - 1] = s[i] - '0';
  if (a[0] % 2 == 0)
    cout << 1 << endl;
  else
    cout << 0 << endl;
  int sum = 0;
  for (i = 0; i < size; i++)
    sum += a[i];
  if (sum % 3 == 0)
    cout << 1 << endl;
  else
    cout << 0 << endl;
  if (div(4) != 0)
    cout << 0 << endl;
  else
    cout << 1 << endl;
  if (div(8) != 0)
    cout << 0 << endl;
  else
    cout << 1 << endl;
  cout << 1 << endl;
  return 0;
}