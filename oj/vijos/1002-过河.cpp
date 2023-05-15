#include <iostream>
#include <string.h>
using namespace std;
char feet[105003];
int stone[100];
int bridge;
int small, big;
int stoneSize;
void go()
{
  int i, j;
  for (i = bridge - big - 1; i >= 0; i--)
  {
    int mm = stoneSize;
    for (j = i + small; j < i + big + 1 && j < bridge; j++)
    {
      if (mm > feet[j])
      {
        mm = feet[j];
      }
    }
    feet[i] += mm;
  }
  cout << (int)feet[0] << endl;
}
void sort(int from, int to)
{
  if (from >= to)
    return;
  int i = from, j = to;
  int k = stone[i];
  while (true)
  {
    while (stone[j] > k)
      j--;
    if (j == i)
      break;
    stone[i] = stone[j];
    stone[j] = k;
    i++;
    while (stone[i] < k)
      i++;
    if (j == i)
      break;
    stone[j] = stone[i];
    stone[i] = k;
    j--;
  }
  sort(from, i - 1);
  sort(i + 1, to);
}
void cut()
{
  int i;
  stone[0] = stone[0] % 105;
  for (i = 1; i < stoneSize; i++)
  {
    stone[i] = stone[i - 1] + (stone[i] - stone[i - 1]) % 105;
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> bridge >> small >> big >> stoneSize;
  int i;
  for (i = 0; i < stoneSize; i++)
  {
    cin >> stone[i];
  }
  sort(0, stoneSize - 1);
  cut();
  memset(feet, 0, sizeof(feet));
  for (i = 0; i < stoneSize; i++)
  {
    feet[stone[i]] = 1;
  }
  bridge = stone[stoneSize - 1] + 1;
  go();
  return 0;
}