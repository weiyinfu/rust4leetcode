#include <iostream>
#include <string.h>
#include <stdio.h>
using namespace std;
struct Node
{
  Node()
  {
    memset(son, 0, sizeof(son));
    isword = false;
  }
  Node *son[26];
  bool isword;
};
int size;
Node *root;
void insert(char word[])
{
  int i;
  Node *now = root;
  for (i = 0; word[i]; i++)
  {
    if (now->son[word[i] - 'a'] == 0)
      now->son[word[i] - 'a'] = new Node();
    now = now->son[word[i] - 'a'];
  }
  now->isword = true;
}
int deep(Node *r)
{
  int i;
  int ans = 0;
  for (i = 0; i < 26; i++)
  {
    if (r->son[i])
    {
      int sondeep = deep(r->son[i]);
      if (sondeep > ans)
        ans = sondeep;
    }
  }
  if (r->isword)
  {
    ans++;
  }
  return ans;
}
int main()
{
  root = new Node();
  cin >> size;
  int i;
  char word[77];
  for (i = 0; i < size; i++)
  {
    cin >> word;
    insert(word);
  }
  cout << deep(root) << endl;
  return 0;
}