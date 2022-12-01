#!/bin/sh


for i in {3..25}
do
  echo "d" | ~/.scripts/cpp_ps.sh "Day$i"
  cp Makefile Day$i
  touch "Day$i"/input.txt
done

