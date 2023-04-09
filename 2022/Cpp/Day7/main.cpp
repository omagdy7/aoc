#include<bits/stdc++.h>

using namespace std;

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

bool isDir(string &s) {
  return s[0] == 'd';
}

bool isFile(string &s) {
  return s[0] >= '0' && s[0] <= '9';
}

long long sumDir(string fileName, map<string, vector<string>> &dirContent) {
  if (isFile(fileName)) {
    int firstSpace = fileName.find(' ');
    cout << fileName.substr(0, firstSpace) << '\n';
    return stoi(fileName.substr(0, firstSpace));
  } else {
    long long sum = 0;
    for (auto subfile : dirContent[fileName]) {
      // cout << subfile << '\n';
      if (isFile(subfile)) {
        sum += sumDir(subfile, dirContent);
      } else {
        sum += sumDir(subfile, dirContent);
      }
    }
    return sum;
  }
}
 
int main() {
  string s;
  vector<string> dirs;
  map<string, long long> mp;
  map<string, vector<string>> dirContent;
  string cwd = "/";
  vector<string> lines;
 
  while(getline(cin, s)) {
    lines.push_back(s);
  }
 
  for (int i = 0; i < lines.size(); i++) {
    s = lines[i];
    if (s[0] == '$') {
      // it's a command
      if (s[2] == 'l') {
        // ls command
        string prevline = lines[i - 1];
        string tmp = prevline.substr(prevline.find(' ', 2) + 1);
        cwd = tmp;
        // cout << cwd << '\n';
      } 
    } else {
      // it's a file or a dir
      string prevline = lines[i - 1];
      if (isDir(s)) {
        string tmp = s.substr(s.find(' ') + 1);
        dirContent[cwd].push_back(tmp);
      } else if (isFile(s)) {
        dirContent[cwd].push_back(s);
        // cout << "file size: " << fsize << '\n';
        // mp[cwd] += fsize;
      }
    } 
  }

  long long ans = 0;
  for (auto [dir, ls] : dirContent) {
    cout << "cwd: " << dir << '\n';
    cout << "\t\t";
    printv(ls);
    // long long sum = sumDir(dir, dirContent);
    // if (sum <= 100000) {
    //   ans += sum;
    // }
  }
  cout << ans << '\n';
}

