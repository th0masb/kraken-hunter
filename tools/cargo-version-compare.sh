#!/usr/bin/env bash

if [ -z "$PROJECT" ];
then
  echo "Must specify \$PROJECT input!"
  exit 1
fi

MAIN_VERSION="$(
  git diff origin/main -- "$PROJECT"/Cargo.toml |
  grep -E '^[-]version = "[0-9]+\.[0-9]+\.[0-9]+"$' |
  grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'
)"
BRANCH_VERSION="$(
  git diff origin/main -- "$PROJECT"/Cargo.toml |
  grep -E '^[+]version = "[0-9]+\.[0-9]+\.[0-9]+"$' |
  grep -Eo '[0-9]+\.[0-9]+\.[0-9]+'
)"

if [[ -z "$BRANCH_VERSION" ]];
then
  echo "Error: No valid version diff in $PROJECT/Cargo.toml found on this branch!"
  exit 1
elif [[ -z "$MAIN_VERSION" && "$BRANCH_VERSION" ]];
then
  echo "Nothing on main branch to compare to! Branch version is $BRANCH_VERSION"
  exit 0
elif [[ "$(printf "%s\n%s" "$MAIN_VERSION" "$BRANCH_VERSION" | sort -uV | head -n1 )" = "$BRANCH_VERSION" ]];
then
  echo "Error: Branch version of $PROJECT ($BRANCH_VERSION) is not bigger than main version ($MAIN_VERSION)!"
  exit 1
else
  echo "Diff shows version has been incremented from $MAIN_VERSION (main) to $BRANCH_VERSION (branch)!"
  exit 0
fi
