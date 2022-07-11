#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
int prime[25000];
int psize;
void init()
{
  bool a[50000];
  memset(a, -1, sizeof(a));
  int i, j;
  psize = 0;
  for (i = 2; i < 50000; i++)
  {
    if (a[i])
    {
      prime[psize++] = i;
    }
    for (j = 0; j < psize && prime[j] * i < 50000; j++)
    {
      a[prime[j] * i] = false;
      if (i % prime[j] == 0)
        break;
    }
  }
}
int main()
{
  long long int n;
  cin >> n;
  init();
  int i;
  for (i = 0; i < psize; i++)
  {
    if (n % prime[i] == 0)
    {
      cout << n / prime[i] << endl;
      break;
    }
  }
  return 0;
}