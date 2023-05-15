#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
char a[1030];
int size;
void see(int from, int to)
{
  int i;
  bool has0, has1;
  has0 = has1 = false;
  for (i = from; i <= to; i++)
  {
    if (a[i] == '0')
    {
      has0 = true;
      if (has1)
      {
        cout << 'F';
        return;
      }
    }
    else
    {
      has1 = true;
      if (has0)
      {
        cout << 'F';
        return;
      }
    }
  }
  if (has0)
    cout << 'B';
  else
    cout << 'I';
}
void go(int from, int to)
{
  if (from == to)
  {
    see(from, from);
    return;
  }
  int len = (to - from + 1) / 2;
  go(from, from + len - 1);
  go(to - len + 1, to);
  see(from, to);
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size >> a;
  go(0, (1 << size) - 1);
  return 0;
}