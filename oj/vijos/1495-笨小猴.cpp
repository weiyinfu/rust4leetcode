#include <iostream>
#include <string.h>
using namespace std;
char a[103];
int c[26];
bool is(int n)
{
  if (n == 0 || n == 1)
    return false;
  for (int i = 2; i < n; i++)
    if (n % i == 0)
      return false;
  return true;
}
int main()
{
  memset(c, 0, sizeof(c));
  cin >> a;
  int i;
  for (i = 0; a[i] != 0; i++)
  {
    c[a[i] - 'a']++;
  }
  int ma = 0, mi = 101;
  for (i = 0; i < 26; i++)
  {
    if (c[i] == 0)
      continue;
    if (ma < c[i])
      ma = c[i];
    if (mi > c[i])
      mi = c[i];
  }
  ma -= mi;
  if (is(ma))
  {
    cout << "Lucky Word" << endl
         << ma << endl;
  }
  else
    cout << "No Answer" << endl
         << 0 << endl;
  return 0;
}