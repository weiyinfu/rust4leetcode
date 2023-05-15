#include <iostream>
using namespace std;
int main()
{
  int size;
  cin >> size;
  int x, y;
  int i;
  for (i = 0; i < size; i++)
    cin >> x >> y;
  int head = 0, rear = size - 1;
  while (1)
  {
    cout << 1 + head++ << endl;
    if (head > rear)
      break;
    cout << 1 + rear-- << endl;
    if (head > rear)
      break;
  }
  return 0;
}