#include<bits/stdc++.h>
using namespace std;

using vi = vector<int>;

#define all(x) (x).begin(), (x).end()
#define rall(x) (x).rbegin(), (x).rend()

void part1() {
  string s;
  int sum = 0;
  while(cin >> s) {
    vector<bool> vis(54, 0);
    string l = s.substr(0, s.size() / 2);
    string r = s.substr(s.size() / 2, s.size());
    for (int i = 0; i < s.size() / 2; i++) {
      if (count(all(r), l[i])) {
        if (isupper(l[i]) && !vis[l[i] - 'A' + 27]) {
          sum += l[i] - 'A' + 27;
          vis[l[i] - 'A' + 27] = 1;
        } else if (islower(l[i]) && !vis[l[i]- 'a' + 1]) {
          sum += l[i] - 'a' + 1;
          vis[l[i] - 'a' + 1] = 1;
        }
      }
    }
  }
  cout << sum << '\n';
}

void getFrqThreeAndSum(string s, int &sum) {
  vi fq(54, 0);
  for (auto x : s) {
    if(isupper(x)) {
      fq[x - 'A' + 27]++;
    } else {
      fq[x - 'a' + 1]++;
    }
  }
  for (int i = 0; i < fq.size(); i++) {
    if (fq[i] == 3) {
      sum += i;
    }
  }
}

void part2() {
  string s;
  int sum = 0;
  vector<string> v;
  while(cin >> s) {
    sort(all(s));
    auto res = unique(all(s));
    v.push_back(string(s.begin(), res));
  }
  string tmp = "";
  int cnt = 1;
  for (int i = 0; i < v.size(); i+=3) {
    tmp = v[i] + v[i + 1] + v[i + 2];
    getFrqThreeAndSum(tmp, sum);
  }
  cout << sum << '\n';
}

int main () {
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);
  // part1();
  part2();
}


