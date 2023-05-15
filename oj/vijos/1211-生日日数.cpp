#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int a[] = {0, 31, 0, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
int day(int year, int month)
{
  if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0)
  {
    if (month == 2)
      return 29;
    else
      return a[month];
  }
  else if (month == 2)
    return 28;
  else
    return a[month];
}
int main()
{
  // freopen("in.txt", "r", stdin);
  int y, m, d;
  cin >> y >> m >> d;
  int i;
  for (i = 0; i < 10000; i++)
  {
    if (d == day(y, m))
    {
      d = 1;
      if (m == 12)
      {
        m = 1;
        y++;
      }
      else
      {
        m++;
      }
    }
    else
      d++;
  }
  cout << y << "-" << m << "-" << d << endl;
  return 0;
}