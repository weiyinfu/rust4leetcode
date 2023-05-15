#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
int main()
{
  // freopen("in.txt", "r", stdin);
  int k;
  double now = 0;
  cin >> k;
  int i = 1;
  while (true)
  {
    now += (1.0 / i);
    if (now > k)
      break;
    i++;
  }
  cout << i << endl;
  return 0;
}