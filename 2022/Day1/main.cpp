#include<bits/stdc++.h>
using namespace std;

using ll = long long;
using vll = vector<long long>;

#define pb push_back 
#define all(x) (x).begin(), (x).end()
#define rall(x) (x).rbegin(), (x).rend()

void solve() {
  string s;
  ll sum = 0;
  ll mx = 0;
  vll v;
  while(getline(cin, s)) {
    if (s.empty()) {
      v.pb(sum);
      sum = 0;
    } else {
      int x = stoi(s);
      sum += x;
    }
  }
  sort(rall(v));
  cout << v[0] + v[1] + v[2] << '\n';
}

int main () {
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);
  solve();
}


