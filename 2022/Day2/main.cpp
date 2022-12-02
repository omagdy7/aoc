#include<bits/stdc++.h>
#include <cstdio>
#include <sstream>
using namespace std;

using ll = long long;
using pi = pair<int, int>;
using vpi = vector<pi>;
using vi = vector<int>;
using vll = vector<long long>;
using mpii = map<int, int>;
using mpll = map<ll, ll>;
using db = long double;

#define pb push_back 
#define all(x) (x).begin(), (x).end()
#define rall(x) (x).rbegin(), (x).rend()
#define lb lower_bound
#define ub upper_bound

const int MOD = (int)1e9 + 7;
const db PI = acos((db)-1);
const int dx[4]{1, 0, -1, 0};
const int dy[4]{0, 1, 0, -1};

//pretty printing
template<typename K, typename V>
void printm(const map<K, V> &mp) {
  cerr << "{" << endl;
  for (auto p : mp) {
    cerr << "  { " << p.first << " : " << p.second << " }\n";
  }
  cerr << "}" << endl;
}
template<typename T>
void printv(const vector<T> &v) {
  cerr << "[";
  for (int i = 0; i < v.size(); i++) {
    if (i == v.size() - 1) {
      cerr << v[i];
    } else {
      cerr << v[i] << ", ";
    }
  }
  cerr << "]\n";
}

template<typename T>
void printvv(const vector<vector<T>> &v) {
  cerr << "[\n";
  for (auto &vec : v) {
    cout << "  ";
    printv(vec);
  }
  cerr << "]\n";
}
void print() {
  cerr << "\n";
}

template<typename T, typename... TS>
void print(T val, TS... vals) {
  cerr << val << " ";
  print(vals...);
}

void solvePart2() {
  char n, m;
  int ans = 0;
  int rockPoints = 1, paperPoints = 2, scissorsPoints = 3;
  int win = 6, draw = 3, loss = 0;
  // y = B = paper
  // x = A = rock
  // z = C = scissors
  while(cin >> n >> m) {
    if (n == 'A')  {
      switch(m) {
        case 'Y': {
          ans += rockPoints + draw;
          break;
        }
        case 'X': {
          ans += scissorsPoints + loss;
          break;
        }
        case 'Z': {
          ans += paperPoints + win;
          break;
        }
      }
    } else if (n == 'B') {
        switch(m) {
          case 'Y': {
            ans += paperPoints + draw;
            break;
          }
          case 'X': {
            ans += rockPoints + loss;
            break;
          }
          case 'Z': {
            ans += scissorsPoints + win;
            break;
          }
        }
    } else if (n == 'C') {
        switch(m) {
          case 'Y': {
            ans += scissorsPoints + draw;
            break;
          }
          case 'X': {
            ans += paperPoints + loss;
            break;
          }
          case 'Z': {
            ans += rockPoints + win;
            break;
          }
        }
    }
  }
  cout << "Part2: " << ans << '\n';
}

void solvePart1() {
  char n, m;
  int ans = 0;
  int rockPoints = 1, paperPoints = 2, scissorsPoints = 3;
  int win = 6, draw = 3, loss = 0;
  // y = B = paper
  // x = A = rock
  // z = C = scissors
  while(cin >> n >> m) {
    if (n == 'A')  {
      switch(m) {
        case 'Y': {
          ans += paperPoints + win;
          break;
        }
        case 'X': {
          ans += rockPoints + draw;
          break;
        }
        case 'Z': {
          ans += scissorsPoints + loss;
          break;
        }
      }
    } else if (n == 'B') {
        switch(m) {
          case 'Y': {
            ans += paperPoints + draw;
            break;
          }
          case 'X': {
            ans += rockPoints + loss;
            break;
          }
          case 'Z': {
            ans += scissorsPoints + win;
            break;
          }
        }
    } else if (n == 'C') {
        switch(m) {
          case 'Y': {
            ans += paperPoints + loss;
            break;
          }
          case 'X': {
            ans += rockPoints + win;
            break;
          }
          case 'Z': {
            ans += scissorsPoints + draw;
            break;
          }
        }
    }
  }
  cout << "Part1: " << ans << '\n';
}

int main () {
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);
  solvePart1();
  // solvePart2();
}


