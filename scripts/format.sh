#!/bin/bash

for file in "$(find $(pwd) | grep "\\.rs$")"
do
  printf "$file\n"
  rustfmt $file
done
