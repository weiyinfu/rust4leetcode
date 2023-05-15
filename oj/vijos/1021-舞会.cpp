#include <iostream>
#include <string.h>
#include <math.h>
#include <stdio.h>
using namespace std;
int main()
{
  // freopen("in.txt", "r", stdin);
  int ren, k;
  cin >> ren >> k;
  int i;
  int count = 0;
  int temp;
  for (i = 0; i < ren; i++)
  {
    int j;
    int know = 0;
    while (cin >> temp && temp != 0)
      know++;
    if (know >= k)
      count++;
  }
  cout << count << endl;
  return 0;
}
