#!/bin/sh

read N

echo "d" | ~/.scripts/cpp_ps.sh "Day$N"
cp Makefile Day$N
touch "Day$N"/input.txt

