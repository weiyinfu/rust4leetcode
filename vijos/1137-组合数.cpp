#include <iostream>
#include <string>
#include <string.h>
#include <math.h>
#include <stdlib.h>
using namespace std;
int prime[10000];
int primeSize = 0;
int cnt[100001];
void init()
{
  bool a[100001];
  memset(a, -1, sizeof(a));
  for (int i = 2; i < 100000; i++)
  {
    if (a[i])
      prime[primeSize++] = i;
    for (int j = 0; j < primeSize && prime[j] * i < 100000; j++)
    {
      a[prime[j] * i] = false;
      if (i % prime[j] == 0)
        break;
    }
  }
}
int main()
{
  init();
  long long int n, m;
  cin >> n >> m;
  if ((m << 1) > n)
    m = n - m;
  memset(cnt, 0, sizeof(cnt));
  for (int i = 0; i < m; i++)
  {
    int num = n - i;
    for (int j = 0; j < 67; j++)
    {
      while (num % prime[j] == 0)
      {
        num /= prime[j];
        cnt[prime[j]]++;
      }
    }
    if (num != 1)
      cnt[num]++;
  }
  for (int i = 2; i <= m; i++)
  {
    int num = i;
    for (int j = 0; j < 67; j++)
    {
      while (num % prime[j] == 0)
      {
        num /= prime[j];
        cnt[prime[j]]--;
      }
    }
    if (num != 1)
      cnt[num]--;
  }
  int ans = 0;
  for (int i = 0; i < primeSize; i++)
  {
    if (cnt[prime[i]])
      ans++;
  }
  cout << ans << endl;
  return 0;
}