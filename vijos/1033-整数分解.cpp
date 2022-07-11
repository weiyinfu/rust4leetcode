#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
struct big
{
  int a[509];
};

void mul(big &b, int k)
{
  int i;
  int enter = 0;
  for (i = 0; i < 250; i++)
  {
    b.a[i] *= k;
    b.a[i] += enter;
    enter = b.a[i] / 10;
    b.a[i] %= 10;
  }
}
big mul(big a, big b)
{
  big c;
  memset(&c, 0, sizeof(c));
  int i, j;
  for (i = 0; i < 250; i++)
  {
    for (j = 0; j < 250; j++)
    {
      c.a[i + j] += a.a[i] * b.a[j];
    }
  }
  for (i = 0; i < 250; i++)
  {
    c.a[i + 1] += c.a[i] / 10;
    c.a[i] %= 10;
  }
  return c;
}
big power(int k)
{
  if (k == 0)
  {
    big a;
    memset(&a, 0, sizeof(a));
    a.a[0] = 1;
    return a;
  }
  big b = power(k >> 1);
  big bb = mul(b, b);
  if (k & 1)
    mul(bb, 3);
  return bb;
}
void show(big a)
{
  int i;
  for (i = 250; a.a[i] == 0; i--)
    ;
  while (i >= 0)
    cout << a.a[i--];
}
int main()
{
  int n;
  cin >> n;
  if (n < 4)
  {
    cout << n << endl;
    return 0;
  }
  int t = n % 3;
  int k = n / 3;
  big ans = power(k - 1);
  if (t == 0)
    mul(ans, 3);
  if (t == 1)
    mul(ans, 4);
  if (t == 2)
    mul(ans, 6);
  show(ans);
  return 0;
}