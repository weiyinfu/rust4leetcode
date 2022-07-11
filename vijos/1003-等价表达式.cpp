#include <iostream>
using namespace std;
#define big 10007
#define test 8
char ti[51];
char choose[26][51];
int size;
int result[11];
bool prior(char a, char b)
{
  if (a == '^')
    return true;
  if (b == '^')
    return false;
  if (a == '*')
    return true;
  if (b == '*')
    return false;
  return true;
}
int power(int a, int b)
{
  int i;
  int ans = 1;
  for (i = 0; i < b; i++)
  {
    ans *= a;
    ans %= big;
  }
  return ans;
}
int op(int a, int b, char o)
{
  switch (o)
  {
  case '^':
    return power(a, b);
  case '*':
    a %= big;
    b %= big;
    a *= b;
    a %= big;
    return a;
  case '+':
    return (a + b) % big;
  case '-':
    return (a - b) % big;
  }
}
int calculate(int x[], int xsize, char o[], int osize)
{
  int i;
  int xstack[51];
  int xtop = 0;
  char ostack[51];
  int otop = 0;
  int xi, oi;
  xi = oi = 0;
  xstack[xtop++] = x[xi++];
  while (xi < xsize && oi < osize)
  {
    while (otop != 0 && prior(ostack[otop - 1], o[oi]))
    {
      xstack[xtop - 2] = op(xstack[xtop - 2], xstack[xtop - 1], ostack[otop - 1]);
      xtop--;
      otop--;
    }
    ostack[otop++] = o[oi++];
    xstack[xtop++] = x[xi++];
  }
  while (otop > 0 && xtop > 0)
  {
    xstack[xtop - 2] = op(xstack[xtop - 2], xstack[xtop - 1], ostack[otop - 1]);
    xtop--;
    otop--;
  }
  return xstack[0];
}
int go(char ex[], int a)
{
  int i, j;
  int x[51];
  int xi = 0;
  char o[51];
  int oi = 0;
  i = 0;
  while (ex[i])
  {
    while (ex[i] == ' ')
      i++;
    if (ex[i] == 0)
      break;
    if (ex[i] == '(')
    {
      int left = 1;
      char temp[51];
      j = 0;
      i++;
      while (true)
      {
        temp[j] = ex[i];
        if (temp[j] == '(')
          left++;
        else if (temp[j] == ')')
          left--;
        if (left == 0)
        {
          temp[j] = 0;
          x[xi++] = go(temp, a);
          i++;
          break;
        }
        i++;
        j++;
        // if (ex[i] == 0)break;
      }
    }
    else if (ex[i] == 'a')
    {
      x[xi++] = a;
      i++;
    }
    else
    {
      int k = 1;
      if (ex[i] == '-')
      {
        k = -1;
        i++;
      }
      else if (ex[i] == '+')
      {
        k = -1;
        i++;
      }
      int n = 0;
      while (ex[i] >= '0' && ex[i] <= '9')
      {
        n *= 10;
        n += ex[i] - '0';
        i++;
      }
      x[xi++] = n * k;
    }
    while (ex[i] == ' ')
      i++;
    if (ex[i] == 0)
      break;
    if (ex[i] == '+' || ex[i] == '-' || ex[i] == '*' || ex[i] == '^')
    {
      o[oi++] = ex[i];
    }
    else if (ex[i] == 0)
      break;
    i++;
  }
  return calculate(x, xi, o, oi);
}
int main()
{
  // freopen("in.txt", "r", stdin);
  cin.getline(ti, sizeof(ti));
  cin >> size;
  cin.getline(choose[0], 55);
  int i;
  for (i = 0; i < size; i++)
  {
    cin.getline(choose[i], 55);
  }
  for (i = 0; i < test; i++)
  {
    result[i] = go(ti, i);
  }
  for (i = 0; i < size; i++)
  {
    int j;
    int ans;
    for (j = 0; j < test; j++)
    {
      ans = go(choose[i], j);
      if (ans != result[j])
        break;
    }
    if (j == test)
    {
      cout << (char)(i + 'A');
    }
  }
  return 0;
}