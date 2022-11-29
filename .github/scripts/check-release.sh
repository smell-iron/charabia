#!/bin/sh

# Checking if current tag matches the package version
current_tag=$(echo $GITHUB_REF | tr -d 'refs/tags/v')
file1='Cargo.toml'

file_tag1=$(grep '^version = ' $file1 | cut -d '=' -f 2 | tr -d '"' | tr -d ' ')
if [ "$current_tag" != "$file_tag1" ]; then
  echo "Error: the current tag does not match the version in package file(s)."
  echo "$file1: found $file_tag1 - expected $current_tag"
  exit 1
fi

echo 'OK'
exit 0
