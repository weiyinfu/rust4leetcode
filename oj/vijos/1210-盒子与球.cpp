#include <iostream>
#include <string.h>
using namespace std;
long long int a[13][13];
int box, ball;
void init()
{
  int i, j;
  a[0][0] = 1;
  for (i = 1; i <= ball; i++)
  {
    for (j = 1; j <= box && j <= i; j++)
    {
      long long int c = 1;
      int k;
      for (k = 1; k <= i + 1 - j; k++)
      {
        c *= (i + 1 - k);
        c /= k;
        a[i][j] += c * a[i - k][j - 1];
      }
    }
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  memset(a, 0, sizeof(a));
  cin >> ball >> box;
  init();
  cout << a[ball][box] << endl;
  return 0;
}
