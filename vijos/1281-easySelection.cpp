#include <iostream>
#include <stdio.h>
using namespace std;
int main()
{
  //	freopen("in.txt", "r", stdin);
  char name[][10] = {"wind", "lolanv"};
  int t;
  cin >> t;
  int size, who;
  while (t-- > 0)
  {
    cin >> size >> who;
    int i, j;
    for (i = 0; i < size; i++)
      scanf("%d", &j);
    cout << name[who] << endl;
  }
  return 0;
}