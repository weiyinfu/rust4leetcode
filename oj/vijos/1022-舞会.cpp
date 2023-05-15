#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
int size;
bool a[205][205];
bool visited[205];
void init()
{
  int i, j, k;
  for (i = 1; i <= size; i++)
  {
    for (j = 1; j <= size; j++)
    {
      for (k = 1; k <= size; k++)
      {
        if (a[j][i] && a[i][k])
        {
          a[j][k] = true;
        }
      }
    }
  }
  for (i = 1; i <= size; i++)
  {
    for (j = 1; j <= size; j++)
    {
      if (a[j][i] != a[i][j])
      {
        a[j][i] = a[i][j] = false;
      }
    }
  }
}
void see(int n)
{
  int i;
  for (i = 1; i <= size; i++)
  {
    if (a[n][i] && !visited[i])
    {
      visited[i] = true;
      see(i);
    }
  }
}
void go()
{
  memset(visited, 0, sizeof(visited));
  int i;
  for (i = 1; i <= size; i++)
  {
    if (!visited[i])
    {
      visited[i] = true;
      see(i);
      visited[i] = false;
    }
  }
  int ans = 0;
  for (i = 1; i <= size; i++)
    if (!visited[i])
      ans++;
  cout << ans << endl;
}
int main()
{
  cin >> size;
  int i;
  memset(a, 0, sizeof(a));
  for (i = 1; i <= size; i++)
  {
    int man;
    while (cin >> man && man != 0)
      a[i][man] = true;
  }
  init();
  go();
  return 0;
}