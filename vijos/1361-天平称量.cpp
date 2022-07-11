#include <iostream>
#include <string.h>
using namespace std;
int size;
char ans[103];
int choose[103][4];
int res[207];
void mul(int k)
{
  int i;
  for (i = 0; i < 207; i++)
  {
    res[i] *= k;
  }
  for (i = 0; i < 206; i++)
  {
    res[i + 1] += res[i] / 10;
    res[i] %= 10;
  }
}
void output()
{
  int point = 2 * size;
  int right = 0;
  while (res[right] == 0 && right < 207)
    right++;
  if (right == 207)
  {
    cout << 0 << endl;
    return;
  }
  if (res[point] == 1)
  {
    cout << 1 << endl;
  }
  else
  {
    int i;
    cout << 0 << '.';
    for (i = point - 1; i >= right; i--)
    {
      cout << res[i];
    }
  }
}
int main()
{
  cin >> size;
  int i, j;
  cin >> ans;
  char c;
  for (i = 0; i < size; i++)
  {
    for (j = 0; j < 4; j++)
    {
      cin >> choose[i][j];
      cin >> c;
    }
  }
  memset(res, 0, sizeof(res));
  res[0] = 1;
  for (i = 0; i < size; i++)
  {
    mul(choose[i][ans[i] - 'A']);
  }
  output();
  return 0;
}