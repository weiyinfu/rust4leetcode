#include <iostream>
#include <string>
using namespace std;
int main()
{
  // freopen("in.txt", "r", stdin);
  int n;
  cin >> n;
  int t = (1 << 30);
  int ans = (n & t);
  while (t)
  {
    ans |= (((ans & t) >> 1) ^ (n & (t >> 1)));
    t /= 2;
  }
  cout << ans + 1 << endl;
  return 0;
}