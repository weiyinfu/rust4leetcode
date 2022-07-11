#include <iostream>
#include <string>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <stdio.h>
using namespace std;
char map[1002][1002];
int line, column;
struct Rectangle
{
  int xmin, ymin, xmax, ymax;
} a[26];
bool graph[26][26];
bool active[26];
char str[27];
int strSize = 0;
void initGraph()
{
  memset(graph, 0, sizeof(graph));
  int i;
  for (i = 0; i < 26; i++)
  {
    if (!active[i])
      continue;
    int x, y;
    for (x = a[i].xmin; x <= a[i].xmax; x++)
    {
      if (map[x][a[i].ymin] != i)
      {
        graph[map[x][a[i].ymin]][i] = true;
      }
      if (map[x][a[i].ymax] != i)
      {
        graph[map[x][a[i].ymax]][i] = true;
      }
    }
    for (y = a[i].ymin; y <= a[i].ymax; y++)
    {
      if (map[a[i].xmax][y] != i)
      {
        graph[map[a[i].xmax][y]][i] = true;
      }
      if (map[a[i].xmin][y] != i)
      {
        graph[map[a[i].xmin][y]][i] = true;
      }
    }
  }
}
bool toNothing(int from)
{
  for (int i = 0; i < 26; i++)
  {
    if (graph[from][i] && active[i])
      return false;
  }
  return true;
}
void go()
{
  int i;
  bool over = true;
  for (i = 0; i < 26; i++)
  {
    if (active[i])
    {
      over = false;
      if (toNothing(i))
      {
        active[i] = false;
        str[strSize++] = i + 'A';
        go();
        strSize--;
        active[i] = true;
      }
    }
  }
  if (over)
  {
    str[strSize] = 0;
    cout << str << endl;
  }
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin >> line >> column;
  char c;
  int i, j;
  memset(map, 0, sizeof(map));
  for (i = 1; i <= line; i++)
  {
    for (j = 1; j <= column; j++)
    {
      cin >> map[i][j];
      if (map[i][j] != '.')
      {
        map[i][j] -= 'A';
      }
    }
  }
  for (i = 0; i < 26; i++)
  {
    a[i].xmin = line + 1;
    a[i].xmax = a[i].ymax = 0;
    a[i].ymin = column + 1;
  }
  for (i = 1; i <= line; i++)
  {
    for (j = 1; j <= column; j++)
    {
      if (map[i][j] != '.')
      {
        if (i < a[map[i][j]].xmin)
          a[map[i][j]].xmin = i;
        if (i > a[map[i][j]].xmax)
          a[map[i][j]].xmax = i;
        if (j < a[map[i][j]].ymin)
          a[map[i][j]].ymin = j;
        if (j > a[map[i][j]].ymax)
          a[map[i][j]].ymax = j;
      }
    }
  }
  for (i = 0; i < 26; i++)
    if (a[i].ymax == 0)
      active[i] = false;
    else
      active[i] = true;
  initGraph();
  go();
  return 0;
}