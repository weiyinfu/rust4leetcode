#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
int m[30], v[30];
int f[30003];
int ge, size;
void go()
{
  int i, j;
  for (i = ge - 1; i >= 0; i--)
  {
    for (j = size; j > 0; j--)
    {
      if (j > m[i])
      {
        if (f[j - m[i]] + v[i] * m[i] > f[j])
        {
          f[j] = f[j - m[i]] + v[i] * m[i];
        }
      }
    }
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size >> ge;
  int i;
  int money, grade;
  memset(f, 0, sizeof(f));
  for (i = 0; i < ge; i++)
  {
    cin >> m[i] >> v[i];
  }
  go();
  cout << f[size] << endl;
  return 0;
}