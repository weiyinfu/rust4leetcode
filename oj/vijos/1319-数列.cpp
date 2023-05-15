#include <iostream>
#include <string>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <stdio.h>
using namespace std;
int main()
{
  int num, which;
  cin >> num >> which;
  int ji = 1;
  int ans = 0;
  while (which)
  {
    if (which & 1)
      ans += ji;
    ji *= num;
    which /= 2;
  }
  cout << ans << endl;
  return 0;
}