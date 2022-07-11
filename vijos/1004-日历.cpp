#include <iostream>
#include <string.h>
using namespace std;
bool a[2002][13][31];
int da[13] = {0, 31, 0, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
int days(int year, int month)
{
  if (year == 2001 && month == 11)
    return 4;
  if (month == 2)
  {
    if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0)
    {
      return 29;
    }
    else
      return 28;
  }
  else
  {
    return da[month];
  }
}
int months(int year)
{
  if (year == 2001)
    return 11;
  else
    return 12;
}
void incDay(int year, int month, int day)
{
  if (year == 2001 && month == 11 && day == 4)
    return;
  int y, m, d;
  if (month == 12 && day == 31)
  {
    y = year + 1;
    m = 1;
    d = 1;
  }
  else if (day == days(year, month))
  {
    y = year;
    m = month + 1;
    d = 1;
  }
  else
  {
    y = year;
    m = month;
    d = day + 1;
  }
  if (a[y][m][d] == false)
  {
    a[year][month][day] = true;
  }
}
void incMonth(int year, int month, int day)
{
  int y, m, d;
  if (year == 2001 && month == 11)
    return;
  if (month == 12)
  {
    y = year + 1;
    m = 1;
    d = day;
  }
  else
  {
    y = year;
    m = month + 1;
    int temp = days(y, m);
    if (day > temp)
      return;
    else
      d = day;
  }
  if (a[y][m][d] == false)
  {
    a[year][month][day] = true;
  }
}
void init()
{
  int y, m, d;
  for (y = 2001; y >= 1900; y--)
  {
    for (m = months(y); m >= 1; m--)
    {
      for (d = days(y, m); d >= 1; d--)
      {
        incDay(y, m, d);
        incMonth(y, m, d);
      }
    }
  }
}
int main()
{
  //	freopen("in.txt", "r", stdin);
  int t;
  int i;
  cin >> t;
  while (t-- > 0)
  {
    memset(a, 0, sizeof(a));
    int year, month, day;
    cin >> year >> month >> day;
    init();
    if (a[year][month][day])
      cout << "YES" << endl;
    else
      cout << "NO" << endl;
  }
  return 0;
}
