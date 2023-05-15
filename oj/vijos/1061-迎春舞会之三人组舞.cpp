#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
int dif = 'a' - 'A';
char a[1000003];
char word[11];
bool has[129];
int size;
int len;
int main()
{
  cin >> word;
  gets(a);
  gets(a);
  int i, j;
  int pos = -1;
  int count = 0;
  for (i = 0; a[i]; i++)
    ;
  a[i] = ' ';
  a[i + 1] = 0;
  i = 0;
  while (1)
  {
    while (a[i] == ' ')
      i++;
    if (a[i] == 0)
      break;
    for (j = 0; word[j]; j++)
    {
      int d = word[j] - a[i + j];
      if (d != dif && d != -dif && d != 0)
        break;
    }
    if (word[j] == 0 && a[i + j] == ' ')
    {
      count++;
      if (pos == -1)
        pos = i;
      i += j;
    }
    else
    {
      for (i += j; a[i] != ' '; i++)
        ;
    }
  }
  if (!~pos)
    cout << -1 << endl;
  else
    cout << count << ' ' << pos << endl;
  return 0;
}