#include <iostream>
#include <string.h>
using namespace std;
int f[1003][105];
int v[105], m[105];
int size, ge;
void go()
{
  int i, j;
  for (i = 1; i <= ge; i++)
  {
    for (j = 1; j <= size; j++)
    {
      if (j >= m[i])
        f[j][i] = f[j - m[i]][i - 1] + v[i];
      if (f[j][i] < f[j][i - 1])
      {
        f[j][i] = f[j][i - 1];
      }
    }
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size >> ge;
  int i;
  for (i = 1; i <= ge; i++)
    cin >> m[i] >> v[i];
  memset(f, 0, sizeof(f));
  go();
  cout << f[size][ge] << endl;
  return 0;
}
