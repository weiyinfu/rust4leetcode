#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
struct point
{
  int x, y;
};
point a[15005];
int ans[15005];
int size;
int cmp(point a, point b)
{
  if (a.x > b.x && a.y > b.y)
    return 1;
  if (a.x < b.x && a.y < b.y)
    return -1;
  if (a.x == b.x)
    return a.y - b.y;
  if (a.y == b.y)
    return a.x - b.x;
  return 0;
}
void go()
{
  int i, j;
  int count;
  for (i = 0; i < size; i++)
  {
    count = 0;
    for (j = 0; j < size; j++)
    {
      if (cmp(a[i], a[j]) > 0)
        count++;
    }
    ans[count]++;
  }
}
int main()
{
  cin >> size;
  int i;
  for (i = 0; i < size; i++)
    cin >> a[i].x >> a[i].y;
  memset(ans, 0, sizeof(ans));
  go();
  for (i = 0; i < size; i++)
  {
    cout << ans[i] << endl;
  }
  return 0;
}