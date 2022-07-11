#include <iostream>
#include <string.h>
using namespace std;
int x[201];
int y[201];
int ans[405];
void tonum(char s[], int n[])
{
  int len = 0;
  while (s[len])
    len++;
  int i;
  for (i = 0; i < len; i++)
  {
    n[i] = s[len - i - 1] - '0';
  }
}
void go()
{
  int i, j;
  for (i = 0; i < 201; i++)
  {
    for (j = 0; j < 201; j++)
    {
      ans[i + j] += x[i] * y[j];
    }
  }
  for (i = 0; i < 201; i++)
    ans[i] += x[i] + y[i];
  for (i = 0; i < 403; i++)
  {
    ans[i + 1] += ans[i] / 10;
    ans[i] %= 10;
  }
}
void output()
{
  int i;
  for (i = 403; ans[i] == 0 && i >= 0; i--)
    ;
  if (i == -1)
  {
    cout << 0 << endl;
  }
  else
  {
    for (; i >= 0; i--)
      cout << ans[i];
  }
}
int main()
{
  char a[201];
  char b[201];
  cin >> a >> b;
  memset(ans, 0, sizeof(ans));
  memset(x, 0, sizeof(x));
  memset(y, 0, sizeof(y));
  tonum(a, x);
  tonum(b, y);
  go();
  output();
  return 0;
}