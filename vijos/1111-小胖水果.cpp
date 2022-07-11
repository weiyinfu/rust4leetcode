#include <iostream>
#include <string.h>
using namespace std;
char a[103];
char b[103];
int asize;
int bsize;
int f[103][103];
void go()
{
  int i, j, k;
  for (i = 1; i < asize; i++)
  {
    for (j = 1; j < bsize; j++)
    {
      if (a[i] == b[j])
      {
        f[i][j] = f[i - 1][j - 1] + 1;
      }
      else if (f[i - 1][j] > f[i][j - 1])
      {
        f[i][j] = f[i - 1][j];
      }
      else
      {
        f[i][j] = f[i][j - 1];
      }
    }
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  while (cin >> (a + 1) >> (1 + b))
  {
    for (asize = 1; a[asize] != 0; asize++)
      ;
    for (bsize = 1; b[bsize] != 0; bsize++)
      ;
    memset(f, 0, sizeof(f));
    go();
    cout << asize + bsize - f[asize - 1][bsize - 1] - 2 << endl;
  }
  return 0;
}