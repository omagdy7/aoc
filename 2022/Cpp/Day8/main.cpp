#include <bits/stdc++.h>
#include <sys/types.h>
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

// pretty printing
template <typename K, typename V> void printm(const map<K, V> &mp) {
  cerr << "{" << endl;
  for (auto p : mp) {
    cerr << "  { " << p.first << " : " << p.second << " }\n";
  }
  cerr << "}" << endl;
}
template <typename T> void printv(const vector<T> &v) {
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

template <typename T> void printvv(const vector<vector<T>> &v) {
  cerr << "[\n";
  for (auto &vec : v) {
    cout << "  ";
    printv(vec);
  }
  cerr << "]\n";
}
void print() { cerr << "\n"; }

template <typename T, typename... TS> void print(T val, TS... vals) {
  cerr << val << " ";
  print(vals...);
}

/* stuff you should look for:
  ---------------------------
   * special cases (n=1?)
   * int overflow, array bounds
   * do smth instead of nothing and stay organized
   * WRITE STUFF DOWN
   * DON'T GET STUCK ON ONE APPROACH
 */

#define u8 uint8_t

struct P2 {
  int x;
  int y;
};

bool isValidIdx(P2 p, int r, int c) {
  return p.x >= 0 && p.y >= 0 && p.x < r && p.y < c;
}

array<int ,4> calcScenicScore(P2 p, vector<vector<int>> &trees) {
  std::array<int, 4> res{1, 1, 1, 1};
  int r = trees.size();
  int c = trees[0].size();

  // down
  for (int i = p.x + 1; i < r; i++) {
    if (trees[i][p.y] >= trees[p.x][p.y] || i == r - 1) {
      res[0] = abs(p.x - i);
      break;
    }
  }
  // up
  for (int i = p.x - 1; i >= 0; i--) {
    if (trees[i][p.y] >= trees[p.x][p.y] || i == 0) {
      res[1] = abs(p.x - i);
      break;
    }
  }

  // right
  for (int i = p.y + 1; i < c; i++) {
    if (trees[p.x][i] >= trees[p.x][p.y] || i == r - 1) {
      // print("in right: ",trees[p.x][i], trees[p.x][p.y]);
      // print("i: ", i, "p.y: ", p.y);
      res[2] = abs(p.y - i);
      break;
    }
  }

  // left
  for (int i = p.y - 1; i >= 0; i--) {
    if (trees[p.x][i] >= trees[p.x][p.y] || i == 0) {
      res[3] = abs(p.y - i);
      break;
    }
  }

  return res;
}

bool isVisibile(P2 p, vector<vector<int>> &trees) {
  std::array<bool, 4> ans{true, true, true, true};
  int r = trees.size();
  int c = trees[0].size();
  // down
  for (int i = p.x + 1; i < r; i++) {
    if (trees[i][p.y] >= trees[p.x][p.y]) {
      ans[0] = false;
      break;
    }
  }
  // up
  for (int i = p.x - 1; i >= 0; i--) {
    if (trees[i][p.y] >= trees[p.x][p.y]) {
      ans[1] = false;
      break;
    }
  }

  // right
  for (int i = p.y + 1; i < c; i++) {
    if (trees[p.x][i] >= trees[p.x][p.y]) {
      ans[2] = false;
      break;
    }
  }

  // left
  for (int i = p.y - 1; i >= 0; i--) {
    if (trees[p.x][i] >= trees[p.x][p.y]) {
      ans[3] = false;
      break;
    }
  }

  for (int i = 0; i < 4; i++) {
    cout << boolalpha << ans[i] << " ";
  }
  cout << endl;
  bool res = 0;
  for (int i = 0; i < 4; i++) {
    res |= ans[i];
  }
  return res;
}

int mulArray(array<int, 4> arr) {
  int ans = 1;
  for (auto x: arr) {
    ans *= x;
  }
  return ans;
}

void solve_part_one() {
  string s;
  vector<vector<int>> trees;
  vector<int> row;
  while (cin >> s) {
    for (auto ch : s) {
      row.pb(ch - '0');
    }
    trees.pb(row);
    row.clear();
  }

  int r = trees.size();
  int c = trees[0].size();
  int ans = 2 * r + 2 * c - 4;
  cout << ans << endl;

  for (int x = 1; x < r - 1; x++) {
    for (int y = 1; y < c - 1; y++) {
      if (isVisibile(P2 { x, y }, trees)) {
        ans++;
      }
    }
  }
  cout << ans << endl;
}

void solve_part_two() {
  string s;
  vector<vector<int>> trees;
  vector<int> row;
  while (cin >> s) {
    for (auto ch : s) {
      row.pb(ch - '0');
    }
    trees.pb(row);
    row.clear();
  }

  int r = trees.size();
  int c = trees[0].size();
  int ans = 0;

  for (int x = 1; x < r - 1; x++) {
    for (int y = 1; y < c - 1; y++) {
      ans = max(ans, mulArray(calcScenicScore(P2 { x, y }, trees)));
    }
  }
  cout << ans << endl;
}

int main() {
  ios_base::sync_with_stdio(false);
  cin.tie(NULL);
  // solve_part_one();
  solve_part_two();
}
