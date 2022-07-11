#include <iostream>
#include <string.h>
using namespace std;
int *f1, *f2, *f3;
void go()
{
  int i;
  memset(f3, 0, sizeof(int) * 301);
  f3[0] = 1;
  for (i = 0; i < 300; i++)
  {
    f3[i] += (f1[i] << 1) + f2[i];
    f3[i + 1] += f3[i] / 10;
    f3[i] %= 10;
  }
  int *temp = f1;
  f1 = f2;
  f2 = temp;
  memcpy(f2, f3, sizeof(int) * 301);
}
void output()
{
  int i;
  for (i = 300; f3[i] == 0; i--)
    ;
  for (; i >= 0; i--)
    cout << f3[i];
}
int main()
{
  f1 = new int[301];
  f2 = new int[301];
  f3 = new int[301];
  memset(f1, 0, sizeof(int) * 301);
  memset(f2, 0, sizeof(int) * 301);
  memset(f3, 0, sizeof(int) * 301);
  f1[0] = 1;
  f2[0] = 2;
  int n;
  cin >> n;
  if (n == 1)
  {
    cout << 1 << endl;
  }
  else if (n == 2)
    cout << 2 << endl;
  else
  {
    n -= 2;
    while (n--)
    {
      go();
    }
    output();
  }
  return 0;
}