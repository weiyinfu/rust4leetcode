#include <iostream>
using namespace std;
char zhong[10];
char hou[10];
void go(int zf, int zt, int hf, int ht)
{
  int i;
  for (i = zf; i <= zt; i++)
    if (zhong[i] == hou[ht])
      break;
  cout << hou[ht];
  if (i > zf)
  {
    go(zf, i - 1, hf, hf + (i - 1) - zf);
  }
  if (i < zt)
  {
    go(i + 1, zt, ht - 1 - (zt - (i + 1)), ht - 1);
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> zhong >> hou;
  int i;
  for (i = 0; hou[i] != 0; i++)
    ;
  go(0, i - 1, 0, i - 1);
  return 0;
}