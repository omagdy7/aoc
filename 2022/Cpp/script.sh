#!/bin/sh

read N

echo "d" | ~/.scripts/cpp_ps.sh "Day$N"
cp Makefile Day$N
touch "Day$N"/input.test
cd "Day$N"
aoc download
sleep 2
mv input input.prod

