#include <iostream>
using namespace std;
int n, x[501][501], cigao[501];
int main()
{
  cin >> n;
  for (int i = 1; i <= n; i++)
    for (int j = i + 1; j <= n; j++)
    {
      cin >> x[i][j];
      x[j][i] = x[i][j];
    }
  for (int i = 1; i <= n; i++)
  {
    int max = 1;
    for (int j = 2; j <= n; j++)
      if (x[i][j] > x[i][max])
        max = j;
    x[i][max] = 0;
    for (int j = 2; j <= n; j++)
      if (x[i][j] > max)
        max = x[i][j];
    cigao[i] = max;
  }
  int max = 0;
  for (int i = 1; i <= n; i++)
    if (cigao[i] > max)
      max = cigao[i];
  cout << 1 << endl
       << max << endl;
  return 0;
}