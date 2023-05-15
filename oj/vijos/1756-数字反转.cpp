#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
int main()
{
  long long int n;
  cin >> n;
  if (n == 0)
  {
    cout << 0 << endl;
    return 0;
  }
  if (n < 0)
  {
    cout << "-";
    n *= -1;
  }
  while (n % 10 == 0)
    n /= 10;
  while (n)
  {
    cout << n % 10;
    n /= 10;
  }
  return 0;
}