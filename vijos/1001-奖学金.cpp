#include <iostream>
using namespace std;
struct student
{
  char name[23];
  int exam;
  int room;
  bool ganbu;
  bool xibu;
  int paper;
  int money;
};
student a[103];
int size;
void go()
{
  int i;
  for (i = 0; i < size; i++)
  {
    a[i].money = 0;
    if (a[i].exam > 80 && a[i].paper > 0)
    {
      a[i].money += 8000;
    }
    if (a[i].exam > 85 && a[i].room > 80)
      a[i].money += 4000;
    if (a[i].exam > 90)
      a[i].money += 2000;
    if (a[i].xibu && a[i].exam > 85)
      a[i].money += 1000;
    if (a[i].room > 80 && a[i].ganbu)
      a[i].money += 850;
  }
}
void run()
{
  int i;
  int sum = 0;
  int who = 0;
  for (i = 0; i < size; i++)
  {
    sum += a[i].money;
    if (a[i].money > a[who].money)
    {
      who = i;
    }
  }
  cout << a[who].name << endl
       << a[who].money << endl
       << sum << endl;
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size;
  int i;
  char c;
  for (i = 0; i < size; i++)
  {
    cin >> a[i].name >> a[i].exam >> a[i].room;
    cin >> c;
    if (c == 'Y')
    {
      a[i].ganbu = true;
    }
    else
    {
      a[i].ganbu = false;
    }
    cin >> c;
    if (c == 'Y')
    {
      a[i].xibu = true;
    }
    else
      a[i].xibu = false;
    cin >> a[i].paper;
  }
  go();
  run();
  return 0;
}