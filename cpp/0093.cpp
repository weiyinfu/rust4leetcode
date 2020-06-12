#include <iostream>
#include <string>
#include <vector>
using namespace std;
void show_vv(vector<vector<int>> pos) {
  for (int i = 0; i < pos.size(); i++) {
    for (int j = 0; j < pos[i].size(); j++) {
      cout << pos[i][j] << ".";
    }
    cout << endl;
  }
}
class Solution {
public:
  string vector_int_to_string(vector<int> a) {
    if (a.size() == 0)
      return string("");
    string s(to_string(a[0]));
    for (int i = 1; i < a.size(); i++) {
      s.append(".");
      s.append(to_string(a[i]));
    }
    return s;
  }
  void show_vec(vector<int> now) {
    cout << "before show vec" << endl;
    cout << vector_int_to_string(now) << endl;
  }
  void go(vector<string> &a, vector<int> now, vector<vector<int>> pos, int p,
          string s) {
    if (now.size() > 4)
      return;
    // cout << vector_int_to_string(now) << " " << p << endl;
    if (p == s.length()) {
      if (now.size() == 4) {
        string ip = vector_int_to_string(now);
        // cout << "found" << ip << endl;
        a.push_back(ip);
      }
      return;
    }
    vector<int> canbe = pos[p];
    for (int i = 0; i < canbe.size(); i++) {
      int end = canbe[i];
      int value = stoi(s.substr(p, end - p));
      now.push_back(value);
      go(a, now, pos, end, s);
      now.pop_back();
    }
  }
  vector<string> restoreIpAddresses(string s) {
    vector<string> a;
    vector<vector<int>> pos;
    for (int i = 0; i < s.length(); i++) {
      vector<int> p;
      pos.push_back(p);
    }
    for (int i = 0; i < s.length(); i++) {
      if (s[i] == '0') {
        pos[i].push_back(i + 1);
        continue;
      }
      for (int j = i + 1; j <= s.length(); j++) {
        string sub = s.substr(i, j - i);
        if (stoi(sub) < 256) {
          pos[i].push_back(j);
        } else {
          break;
        }
      }
    }
    // show_vv(pos);
    // cout << "==" << endl;
    vector<int> now;
    go(a, now, pos, 0, s);
    return a;
  }
};