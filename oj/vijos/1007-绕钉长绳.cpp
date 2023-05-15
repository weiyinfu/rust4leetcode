#include <iostream>
#include <string.h>
#include <math.h>
#include <stdio.h>
using namespace std;
double x[103], y[103];
int size;
double r;
double dis(int i, int j)
{
  double dx = x[i] - x[j];
  double dy = y[i] - y[j];
  return sqrt(dx * dx + dy * dy);
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size >> r;
  int i;
  for (i = 0; i < size; i++)
  {
    cin >> x[i] >> y[i];
  }
  if (size == 1)
  {
    double len = 2 * 3.1415926 * r;
    printf("%.2lf\n", len);
    return 0;
  }
  if (size == 2)
  {
    double len = 2 * dis(0, 1);
    len += 2 * 3.1415926 * r;
    printf("%.2lf\n", len);
    return 0;
  }
  double len = dis(0, size - 1);
  for (i = 0; i < size - 1; i++)
  {
    len += dis(i, i + 1);
  }
  len += 2 * 3.1415926 * r;
  printf("%.2lf\n", len);
  return 0;
}