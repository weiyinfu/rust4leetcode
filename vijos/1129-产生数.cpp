#include <iostream>
#include <string.h>
#include <math.h>
#include <stdio.h>
using namespace std;
bool a[11][11];
char n[31];
char ans[50];
void mul(int k)
{
  int i;
  int enter = 0;
  for (i = 0; i < 49; i++)
  {
    ans[i] *= k;
    ans[i] += enter;
    enter = ans[i] / 10;
    ans[i] %= 10;
  }
}
int main()
{
  int size;
  cin >> n >> size;
  int i, j, k;
  memset(a, 0, sizeof(a));
  for (i = 0; i < size; i++)
  {
    cin >> j >> k;
    a[j][k] = true;
  }
  for (i = 0; i < 10; i++)
    a[i][i] = true;
  for (i = 0; i < 10; i++)
  {
    for (j = 0; j < 10; j++)
    {
      for (k = 0; k < 10; k++)
      {
        if (a[j][i] && a[i][k])
          a[j][k] = true;
      }
    }
  }
  int count[10];
  for (i = 0; i < 10; i++)
  {
    count[i] = 0;
    for (j = 0; j < 10; j++)
    {
      if (a[i][j])
        count[i]++;
    }
  }
  memset(ans, 0, sizeof(ans));
  ans[0] = 1;
  for (i = 0; n[i]; i++)
    mul(count[n[i] - '0']);
  for (i = 49; ans[i] == 0; i--)
    ;
  while (i >= 0)
    cout << (char)(ans[i--] + '0');
  return 0;
}