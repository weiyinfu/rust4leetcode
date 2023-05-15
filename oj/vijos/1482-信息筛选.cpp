#include <iostream>
#include <string.h>
#include <math.h>
#include <stdio.h>
using namespace std;
char a[13];
bool go()
{
  int i, j, p, q;
  for (i = 0; a[i]; i++)
  {
    for (j = i + 1; a[j]; j++)
    {
      for (p = j + 1; a[p]; p++)
      {
        for (q = p + 1; a[q]; q++)
        {
          if (a[i] == a[j] && a[p] == a[q] || a[i] == a[q] && a[p] == a[j])
            return false;
        }
      }
    }
  }
  return true;
}
int main()
{
  // freopen("in.txt", "r", stdin);
  int size;
  cin >> size;
  gets(a);
  while (size--)
  {
    gets(a);
    if (go())
      cout << "safe" << endl;
    else
      cout << "de1ete" << endl;
  }
  return 0;
}