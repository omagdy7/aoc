#include<bits/stdc++.h>
#include <cstdio>
#include <sstream>
using namespace std;

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

void solvePart2() {
  char n, m;
  int ans = 0;
  int rockPoints = 1, paperPoints = 2, scissorsPoints = 3;
  int win = 6, draw = 3, loss = 0;
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


int main () {
	ios_base::sync_with_stdio(false);
	cin.tie(NULL);
  solvePart1();
  // solvePart2();
}


