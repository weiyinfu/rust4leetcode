#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
int a[5005];
int size;
int father(int who)
{
  while (a[who] != -1)
    who = a[who];
  return who;
}
int main()
{
  int edge, ask;
  cin >> size >> edge >> ask;
  int i;
  int from, to;
  memset(a, -1, sizeof(a));
  for (i = 0; i < edge; i++)
  {
    cin >> from >> to;
    int ff = father(from);
    int tf = father(to);
    if (ff != tf)
    {
      a[ff] = tf;
    }
  }
  for (i = 0; i < ask; i++)
  {
    cin >> from >> to;
    int ff = father(from);
    int tf = father(to);
    if (ff != tf)
    {
      cout << "No" << endl;
    }
    else
    {
      cout << "Yes" << endl;
    }
  }
  return 0;
}