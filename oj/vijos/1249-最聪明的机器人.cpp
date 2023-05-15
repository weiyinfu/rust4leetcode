#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
bool f(int n)
{
  return n % 4;
}
int main()
{
  int i;
  for (i = 0; i < 10; i++)
  {
    int a, b;
    cin >> a >> b;
    if (a < b)
    {
      if (f(b))
      {
        cout << 2 << endl;
      }
      else
      {
        cout << 1 << endl;
      }
    }
    else
    {
      if (f(a))
      {
        cout << 1 << endl;
      }
      else
      {
        cout << 2 << endl;
      }
    }
  }
  return 0;
}