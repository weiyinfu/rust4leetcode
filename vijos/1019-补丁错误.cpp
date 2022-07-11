#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
bool visited[32770];
struct Bu
{
  char canUse[17];
  char result[17];
  int time;
};
Bu bu[105];
bool has(int n, int i)
{
  return n & (1 << i);
}
int errorSize, mendSize;
int ans;
int now;
bool canUse(int state, int b)
{
  int i;
  for (i = 0; i < errorSize; i++)
  {
    if (bu[b].canUse[i] == '+' && !has(state, i))
      return false;
    if (bu[b].canUse[i] == '-' && has(state, i))
      return false;
  }
  return true;
}
int use(int state, int b)
{
  int i;
  int ret = state;
  for (i = 0; i < errorSize; i++)
  {
    if (bu[b].result[i] == '+')
      ret |= (1 << i);
    if (bu[b].result[i] == '-')
      ret &= ~(1 << i);
  }
  return ret;
}
void debug(int n)
{
  if (n)
    debug(n >> 1);
  cout << (n & 1);
}
void go(int node, int now)
{
  if (visited[node] || now >= ans)
    return;
  if (node == 0)
  {
    ans = now;
    return;
  }
  visited[node] = true;
  int i;
  for (i = 0; i < mendSize; i++)
  {
    if (canUse(node, i))
    {
      go(use(node, i), now + bu[i].time);
    }
  }
  visited[node] = false;
}
int main()
{
  cin >> errorSize >> mendSize;
  int i;
  for (i = 0; i < mendSize; i++)
  {
    cin >> bu[i].time;
    cin >> bu[i].canUse;
    cin >> bu[i].result;
  }
  memset(visited, 0, sizeof(visited));
  ans = (1 << 31) - 1;
  go((1 << errorSize) - 1, 0);
  if (ans == (1 << 31) - 1)
    cout << 0 << endl;
  else
    cout << ans << endl;
  return 0;
}