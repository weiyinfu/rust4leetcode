#include <iostream>
using namespace std;
int a[10];
int h;
int main()
{
  int i;
  for (i = 0; i < 10; i++)
    cin >> a[i];
  cin >> h;
  h += 30;
  int count = 0;
  for (i = 0; i < 10; i++)
    if (a[i] <= h)
      count++;
  cout << count << endl;
  return 0;
}