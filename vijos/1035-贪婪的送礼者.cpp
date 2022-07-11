#include <iostream>
#include <stdio.h>
#include <string.h>
using namespace std;
char man[20][20];
int recieve[20];
int send[20];
int size;
int numOf(char name[])
{
  int i;
  for (i = 0; i < size; i++)
    if (strcmp(man[i], name) == 0)
      return i;
}
void go()
{
  char name[20];
  int now;
  int peer;
  int i;
  char fname[20];
  int fnum;
  while (cin >> name >> now >> peer)
  {
    if (peer == 0)
    {
      continue;
    }
    int give = now / peer;
    int num = numOf(name);
    send[num] = give * peer;
    for (i = 0; i < peer; i++)
    {
      cin >> fname;
      fnum = numOf(fname);
      recieve[fnum] += give;
    }
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> size;
  int i;
  memset(send, 0, sizeof(send));
  memset(recieve, 0, sizeof(recieve));
  for (i = 0; i < size; i++)
    cin >> man[i];
  go();
  for (i = 0; i < size; i++)
    cout << man[i] << " " << recieve[i] - send[i] << endl;
  return 0;
}