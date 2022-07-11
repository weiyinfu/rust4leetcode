#include <iostream>
#include <string.h>
using namespace std;
int cost[12];
int main()
{
  // freopen("in.txt", "r", stdin);
  int i;
  for (i = 0; i < 12; i++)
    cin >> cost[i];
  int save = 0;
  int now = 0;
  for (i = 0; i < 12; i++)
  {
    now += 300;
    if (now >= cost[i])
    {
      int left = now - cost[i];
      int saving = left / 100;
      save += saving * 100;
      now -= saving * 100;
      now -= cost[i];
    }
    else
    {
      cout << "-" << i + 1 << endl;
      return 0;
    }
  }
  now += (save * 6 / 5);
  cout << now << endl;
  return 0;
}