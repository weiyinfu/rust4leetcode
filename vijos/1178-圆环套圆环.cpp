#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
double c = (sqrt(5) - 1) / 2;
int main()
{
  long long n;
  cin >> n;
  cout << (long long)(c * (n + 1)) << endl;
  return 0;
}