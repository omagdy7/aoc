#include <bits/stdc++.h>

using namespace std;

class Dir {
private:
  string name;
  vector<Dir *> dir_list;
  unsigned int size;
  bool is_file;

public:
  Dir(string _name, unsigned int _size = 0) {
    this->name = _name;
    this->size = _size;

    this->is_file = true;
    if (_size == 0) {
      this->is_file = false;
    }
  }

  void add(Dir *dir) {
    dir_list.emplace_back(dir);
  }
};

class FileSystem {
private:
  Dir root_dir;
  map<string, Dir> map_dir;

public:
  void add_to_dir(string path, Dir* dir) {
    map_dir[path].add(dir);
  }

  void create_dir(string path, Dir dir) {
    if (!map_dir.count(path)) {
      map_dir[path] = dir;
    }
  }
};

void read_input() {
  string cur_dir = "";
}

int main() {
  return 0;
}
