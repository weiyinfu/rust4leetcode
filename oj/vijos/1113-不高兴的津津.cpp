#include <iostream>
#include <string.h>
using namespace std;
int week[7];
int main()
{
  // freopen("in.txt", "r", stdin);
  int i;
  for (i = 0; i < 7; i++)
  {
    int a, b;
    cin >> a >> b;
    week[i] = a + b;
  }
  int ma = week[0];
  int which = 0;
  for (i = 0; i < 7; i++)
  {
    if (ma < week[i])
    {
      ma = week[i];
      which = i;
    }
  }
  cout << which + 1 << endl;
  return 0;
}