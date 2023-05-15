#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
bool a[1005];
int size;
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size;
  memset(a, 0, sizeof(a));
  int i;
  int j;
  int count = 0;
  for (i = 0; i < size; i++)
  {
    cin >> j;
    if (a[j] == false)
    {
      a[j] = true;
      count++;
    }
  }
  cout << count << endl;
  for (i = 0; i < 1003; i++)
  {
    if (a[i])
    {
      cout << i << " ";
    }
  }
  return 0;
}