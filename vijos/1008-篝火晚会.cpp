#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
int a[50005];
int size;
void go()
{
  int i, j;
  int ans = 0;
  int dis[50005];
  memset(dis, 0, sizeof(dis));
  for (i = 1; i <= size; i++)
  {
    int d = (a[i] - i + size) % size;
    dis[d]++;
  }
  for (i = 0; i < size; i++)
  {
    if (ans < dis[i])
      ans = dis[i];
  }
  memset(dis, 0, sizeof(dis));
  for (i = 1; i <= size; i++)
  {
    int d = (a[size + 1 - i] - i + size) % size;
    dis[d]++;
  }
  for (i = 0; i < size; i++)
  {
    if (ans < dis[i])
      ans = dis[i];
  }
  cout << size - ans << endl;
}
int main()
{
  int neibor[50005][2];
  int i;
  cin >> size;
  for (i = 1; i <= size; i++)
  {
    scanf("%d%d", &neibor[i][0], &neibor[i][1]);
  }
  bool used[50005];
  memset(used, 0, sizeof(used));
  a[1] = 1;
  used[1] = true;
  for (i = 2; i <= size; i++)
  {
    int last = a[i - 1];
    if (!used[neibor[last][0]])
    {
      a[i] = neibor[last][0];
      used[a[i]] = true;
    }
    else if (!used[neibor[last][1]])
    {
      a[i] = neibor[last][1];
      used[a[i]] = true;
    }
    else
    {
      cout << -1 << endl;
      return 0;
    }
  }
  if (neibor[1][0] != a[size] && neibor[1][1] != a[size])
  {
    cout << -1 << endl;
    return 0;
  }
  go();
  return 0;
}