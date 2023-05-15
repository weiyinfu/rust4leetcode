#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
int size;
bool a[205][205];
bool call[205];
void visit(int n)
{
  int i;
  for (i = 1; i <= size; i++)
  {
    if (a[n][i])
    {
      if (call[i])
      {
        call[i] = false;
        visit(i);
      }
    }
  }
}
void go()
{
  memset(call, -1, sizeof(call));
  int i;
  for (i = 1; i <= size; i++)
  {
    if (call[i])
    {
      call[i] = false;
      visit(i);
      call[i] = true;
    }
  }
  int ans = 0;
  for (i = 1; i <= size; i++)
    if (call[i])
      ans++;
  cout << ans << endl;
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size;
  int i;
  memset(a, 0, sizeof(a));
  for (i = 1; i <= size; i++)
  {
    int man;
    while (cin >> man && man != 0)
      a[i][man] = true;
  }
  go();
  return 0;
}