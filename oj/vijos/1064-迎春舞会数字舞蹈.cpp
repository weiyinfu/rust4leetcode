#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
bool map[10][5][3] = {
    0, 1, 0,
    1, 0, 1,
    0, 0, 0,
    1, 0, 1,
    0, 1, 0,

    0, 0, 0,
    0, 0, 1,
    0, 0, 0,
    0, 0, 1,
    0, 0, 0,

    0, 1, 0,
    0, 0, 1,
    0, 1, 0,
    1, 0, 0,
    0, 1, 0,

    0, 1, 0,
    0, 0, 1,
    0, 1, 0,
    0, 0, 1,
    0, 1, 0,

    0, 0, 0,
    1, 0, 1,
    0, 1, 0,
    0, 0, 1,
    0, 0, 0,

    0, 1, 0,
    1, 0, 0,
    0, 1, 0,
    0, 0, 1,
    0, 1, 0,

    0, 1, 0,
    1, 0, 0,
    0, 1, 0,
    1, 0, 1,
    0, 1, 0,

    0, 1, 0,
    0, 0, 1,
    0, 0, 0,
    0, 0, 1,
    0, 0, 0,

    0, 1, 0,
    1, 0, 1,
    0, 1, 0,
    1, 0, 1,
    0, 1, 0,

    0, 1, 0,
    1, 0, 1,
    0, 1, 0,
    0, 0, 1,
    0, 1, 0};
int size;
char a[1000];
void heng(int which)
{
  int i, j, k;
  for (i = 0; a[i]; i++)
  {
    cout << ' ';
    cout << ' ';
    char ch = ' ';
    if (map[a[i] - '0'][which][1])
      ch = '-';
    for (j = 0; j < size; j++)
    {
      cout << ch;
    }
    cout << ' ';
  }
}
void shu(int which)
{
  int i, j, k;
  for (j = 0; j < size; j++)
  {
    for (i = 0; a[i]; i++)
    {
      cout << ' ';
      if (map[a[i] - '0'][which][0])
        cout << '|';
      else
        cout << ' ';
      for (k = 0; k < size; k++)
        cout << ' ';
      if (map[a[i] - '0'][which][2])
        cout << '|';
      else
        cout << ' ';
    }
    cout << endl;
  }
}
int main()
{
  cin >> size >> a;
  heng(0);
  cout << endl;
  shu(1);
  heng(2);
  cout << endl;
  shu(3);
  heng(4);
  return 0;
}