#include <iostream>
#include <string>
using namespace std;
int main()
{
  int n;
  cin >> n;
  long long int ans = 1;
  int d = (n << 1);
  for (int i = 1; i <= n; i++)
  {
    ans *= (d + 1 - i);
    ans /= i;
  }
  ans /= (n + 1);
  cout << ans << endl;
  return 0;
}