#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int value[33];
int size;
long long int f[33][33][2];
void init()
{
  int i, j, k;
  for (i = 1; i <= size; i++)
  {
    f[i][i - 1][0] = 1;
  }
  for (i = size; i > 0; i--)
  {
    f[i][i][0] = value[i];
    f[i][i][1] = i;
    for (j = i + 1; j <= size; j++)
    {
      for (k = i; k < j; k++)
      {
        int temp = f[i][k - 1][0] * f[k + 1][j][0] + value[k];
        if (temp > f[i][j][0])
        {
          f[i][j][0] = temp;
          f[i][j][1] = k;
        }
      }
    }
  }
}
void visit(int from, int to)
{
  int m = f[from][to][1];
  cout << m << " ";
  if (m > from)
    visit(from, m - 1);
  if (m < to)
    visit(m + 1, to);
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size;
  int i;
  for (i = 1; i <= size; i++)
    cin >> value[i];
  memset(f, 0, sizeof(f));
  init();
  cout << f[1][size][0] << endl;
  visit(1, size);
  return 0;
}