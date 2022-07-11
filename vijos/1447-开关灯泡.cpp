#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
#define size 205
int n[205], ni;
int ans[205];
int now[205];
int two[205];
int mu[205];
void getTwo()
{
  memset(two, 0, sizeof(two));
  int i;
  for (i = 0; i < size; i++)
  {
    two[i] += (ans[i] << 1);
    two[i + 1] = two[i] / 10;
    two[i] %= 10;
  }
  for (i = size - 2; i >= 0; i--)
    two[i + 1] = two[i];
}
void getNow()
{
  int i;
  for (i = size - 3; i >= 0; i--)
  {
    now[i + 2] = now[i];
  }
  now[1] = n[ni--];
  now[0] = n[ni--];
}
void mul(int k)
{ // mu=two_k*k
  memset(mu, 0, sizeof(mu));
  int i;
  for (i = 0; i < size; i++)
  {
    mu[i] += two[i] * k;
    mu[i + 1] = mu[i] / 10;
    mu[i] %= 10;
  }
}
int cmp()
{ // now-mu
  int i;
  for (i = size - 1; i >= 0; i--)
    if (now[i] != mu[i])
      return now[i] - mu[i];
  return 0;
}
void sub()
{ // now-mu
  int i;
  for (i = 0; i < size - 1; i++)
  {
    now[i] -= mu[i];
    if (now[i] < 0)
    {
      now[i + 1]--;
      now[i] += 10;
    }
  }
}
void getAns()
{
  int i;
  for (i = size - 2; i >= 0; i--)
  {
    ans[i + 1] = ans[i];
  }
  for (i = 9; i >= 0; i--)
  {
    two[0] = i;
    mul(i);
    if (cmp() >= 0)
    {
      ans[0] = i;
      sub();
      return;
    }
  }
}
void go()
{
  while (ni >= 0)
  {
    getTwo();
    getNow();
    getAns();
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  char a[205];
  while (cin >> a)
  {
    int i;
    for (i = 0; a[i]; i++)
      ;
    ni = i;
    memset(n, 0, sizeof(n));
    for (i = 0; i < ni; i++)
      n[i] = a[ni - 1 - i] - '0';
    if ((ni & 1) == 0)
      ni--;
    memset(ans, 0, sizeof(ans));
    memset(now, 0, sizeof(now));
    go();
    for (i = size - 1; ans[i] == 0; i--)
      ;
    for (; i >= 0; i--)
      cout << ans[i];
    cout << endl;
  }
  return 0;
}