#include <iostream>
#include <string>
#include <string.h>
#include <math.h>
#include <stdlib.h>
using namespace std;
int prime[5000];
int primeSize = 0;
void init()
{
  bool a[10000];
  memset(a, 1, sizeof(a));
  for (int i = 2; i < 10000; i++)
  {
    if (a[i])
      prime[primeSize++] = i;
    for (int j = 0; j < primeSize && i * prime[j] < 10000; j++)
    {
      a[prime[j] * i] = false;
      if (i % prime[j] == 0)
        break;
    }
  }
}
int a[21];
int ans = 0;
int size;
bool isPrime(int n)
{
  for (int i = 0; i < primeSize && prime[i] * prime[i] <= n; i++)
  {
    if (n % prime[i] == 0)
      return false;
  }
  return true;
}
void go(int from, int choose, int now)
{
  if (choose == 0)
  {
    if (isPrime(now))
      ans++;
    return;
  }
  if (from >= size)
    return;
  int i, j;
  for (i = from; i < size; i++)
  {
    go(i + 1, choose - 1, now + a[i]);
  }
}
int main()
{
  init();
  int choose;
  cin >> size >> choose;
  int i;
  for (i = 0; i < size; i++)
    cin >> a[i];
  go(0, choose, 0);
  cout << ans << endl;
  return 0;
}