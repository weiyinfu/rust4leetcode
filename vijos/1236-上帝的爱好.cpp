#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
int a[4007];
int len[4007]; // from i how long it takes to form a string
int second[4007];
int size;
int ans;
void xxyy()
{
  int i, j;
  for (i = 0; i < size; i++)
  {
    for (j = second[i] + 1; j < len[i]; j++)
    {
      if (len[i] > second[j])
      {
        len[i] = second[j];
      }
    }
  }
}
void xyxyORxyyx()
{
  int i, j;
  for (i = 0; i < size; i++)
  {
    for (j = i + 1; j < len[i]; j++)
    {
      if (a[i] == a[j])
        continue;
      if (second[j] < second[i])
      {
        if (len[i] > second[i])
          len[i] = second[i];
      }
      else if (second[j] > second[i])
      {
        if (len[i] > second[j])
          len[i] = second[j];
      }
    }
  }
}
void go()
{
  int i = 0;
  ans = 0;
  while (1)
  {
    int mi = i;
    for (; i < len[mi]; i++)
    {
      if (len[i] < len[mi])
        mi = i;
    }
    i = len[mi] + 1;
    if (i <= size)
      ans++;
    if (i >= size)
      break;
  }
}
int main()
{
  cin >> size;
  int i;
  for (i = 0; i < size; i++)
    cin >> a[i];
  memset(len, 0, sizeof(len));
  for (i = 0; i < size; i++)
    len[i] = size;
  int last[10001];
  memset(last, -1, sizeof(last));
  for (i = 0; i < size; i++)
    second[i] = size;
  for (i = 0; i < size; i++)
  {
    if (last[a[i]] != -1)
    {
      second[last[a[i]]] = i;
    }
    last[a[i]] = i;
  }
  xxyy();
  xyxyORxyyx();
  go();
  cout << ans << endl;
  return 0;
}