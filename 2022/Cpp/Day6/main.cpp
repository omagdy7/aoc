#include <bits/stdc++.h>
using namespace std;

int solve(int window) {
  string s;
  cin >> s;
  for (int i = 0; i < s.size() - window; i++) {
    set<char> st;
    for (int j = i; j < i + window; j++) {
      st.insert(s[j]);
    }
    if (st.size() == window) {
      return i + window;
    }
  }
  return -1;
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  cout << "Part1: " << solve(4);
  // cout << "Part2: " << solve(14);
}
