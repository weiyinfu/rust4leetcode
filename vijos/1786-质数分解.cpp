#include <iostream>
#include <vector>
#include <algorithm>
#include <stdio.h>
#include <string.h>
#include <string>
using namespace std;
typedef long long ll;
#define re(i, n) for (int i = 0; i < n; i++)
int main()
{
  // freopen("in.txt", "r", stdin);
  int n;
  cin >> n;
  for (int i = 2; i < n; i++)
  {
    if (n % i == 0)
    {
      cout << n / i << endl;
      break;
    }
  }
  return 0;
}