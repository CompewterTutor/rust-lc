#!/bin/bash

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Usage: $0 <problem_number> <problem_title>"
  exit 1
fi

PROBLEM_NUMBER=$1
PROBLEM_TITLE=$2
PROBLEM_DIR="problem_${PROBLEM_NUMBER}_$(echo $PROBLEM_TITLE | tr ' ' '-')"

# Create the new problem directory
mkdir $PROBLEM_DIR
cd $PROBLEM_DIR

# Initialize a new Cargo package
cargo init --bin

# Update the package name in Cargo.toml
sed -i '' "s/name = \"problem_${PROBLEM_NUMBER}_$(echo $PROBLEM_TITLE | tr ' ' '-')\"/name = \"$PROBLEM_DIR\"/" Cargo.toml

# Go back to the root directory
cd ..

# Add the new problem to the workspace members
sed -i '' "/members = \[/a\\
    \"$PROBLEM_DIR\",
" Cargo.toml

echo "Problem $PROBLEM_NUMBER - $PROBLEM_TITLE added successfully."
