#include <iostream>
#include <string.h>
using namespace std;
bool has[10003];
int a[103];
int main()
{
  // freopen("in.txt", "r", stdin);
  int size;
  cin >> size;
  int i;
  memset(has, 0, sizeof(has));
  for (i = 0; i < size; i++)
  {
    cin >> a[i];
    has[a[i]] = true;
  }
  int ans = 0;
  int j;
  for (i = 0; i < size; i++)
  {
    for (j = i + 1; j < size; j++)
    {
      if (a[i] + a[j] <= 10000)
      {
        if (has[a[i] + a[j]])
        {
          ans++;
          has[a[i] + a[j]] = false;
        }
      }
    }
  }
  cout << ans << endl;
  return 0;
}