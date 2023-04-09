#include<bits/stdc++.h>
using namespace std;

struct P2 {
  int start;
  int end;
};

void solve_part_one() {
  P2 first, second;
  int ans = 0;
  while(cin >> first.start >> first.end >> second.start >> second.end) {
    if ((first.start >= second.start && first.end <= second.end) || (second.start >= first.start && second.end <= first.end)) {
      ans++;
    }
  }
  cout << ans << '\n';
}

void solve_part_two() {
  P2 f, s;
  int ans = 0;
  while(cin >> f.start >> f.end >> s.start >> s.end) {
    vector<int> vis(100, 0);
    for (int i = f.start; i <= f.end; i++) {
      vis[i] += 1;
    }
    for (int i = s.start; i <= s.end; i++) {
      vis[i] += 1;
    }
    for (int i = 0; i <= vis.size(); i++) {
      if(vis[i] == 2) {
        ans++;
        break;
      }
    }
  }
  cout << ans << '\n';
}

int main () {
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);
  solve_part_one();
  // solve_part_two();
}


