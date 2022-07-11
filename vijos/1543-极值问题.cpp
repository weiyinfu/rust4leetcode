#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
int main()
{
  long long int k;
  cin >> k;
  long long int i, j;
  i = j = 1;
  while (1)
  {
    j += i;
    i = j - i;
    if (j > k)
      break;
  }
  i = j - i;
  j = j - i;
  cout << i * i + j * j << endl;
  return 0;
}