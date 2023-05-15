#include <iostream>
#include <string.h>
#include <math.h>
using namespace std;
int card[107];
int size;
int ave;
int ans = 0;
void put(int pos, int ask)
{
  if (pos == size)
    return;
  if (ask)
  {
    card[pos] -= ask;
    ans++;
  }
  put(pos + 1, ave - card[pos]);
}
int main()
{
  cin >> size;
  int i;
  int sum = 0;
  for (i = 0; i < size; i++)
  {
    cin >> card[i];
    sum += card[i];
  }
  ave = sum / size;
  put(0, 0);
  cout << ans << endl;
  return 0;
}