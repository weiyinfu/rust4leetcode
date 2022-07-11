#include <iostream>
#include <math.h>
#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <time.h>
#include <stdlib.h>
using namespace std;
int main()
{
  double l, v;
  cin >> l >> v;
  l *= 2;
  l /= 3;
  printf("%.1f\n%.1f", l, l / v);
  return 0;
}