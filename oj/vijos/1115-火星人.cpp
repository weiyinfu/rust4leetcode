#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
int a[10003];
int size;
void sort(int to)
{
  int i, j;
  for (i = 0; i < to; i++)
  {
    for (j = i + 1; j < to; j++)
    {
      if (a[i] < a[j])
      {
        int temp = a[j];
        a[j] = a[i];
        a[i] = temp;
      }
    }
  }
}
void inc()
{
  int i, j, k;
  for (i = 1; i < size; i++)
  {
    if (a[i - 1] > a[i])
      break;
  }
  for (j = 0; j < i; j++)
  {
    if (a[j] > a[i])
      break;
  }
  int temp = a[i];
  a[i] = a[j];
  a[j] = temp;
  sort(i);
}
int main()
{
  int adding;
  int i, j, k;
  cin >> size >> adding;
  for (i = size - 1; i >= 0; i--)
    cin >> a[i];
  for (i = 0; i < adding; i++)
  {
    inc();
  }
  for (j = size - 1; j >= 0; j--)
    cout << a[j] << " ";
  return 0;
}