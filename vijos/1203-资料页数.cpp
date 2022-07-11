#include <iostream>
using namespace std;
int lines;
int page;
int a[1007];
int ans = 0;
void go()
{
  int i;
  int pa = 0;
  for (i = 1; i <= lines; i++)
  {
    if (pa >= a[i])
    {
      pa -= a[i];
    }
    else
    {
      ans++;
      i--;
      pa = page;
    }
  }
}
int main()
{
  cin >> lines >> page;
  int foot;
  cin >> foot;
  int i;
  int x, y;
  for (i = 0; i < 1007; i++)
    a[i] = 1;
  for (i = 0; i < foot; i++)
  {
    cin >> x >> y;
    a[x] += y;
  }
  go();
  cout << ans << endl;
  return 0;
}