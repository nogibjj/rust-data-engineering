!#/bin/bash

# Merge the bashrc file provided in the repository with the one in the home directory.
cat bashrc >> ~/.bashrc

# Source the bashrc file to apply the changes.
source ~/.bashrc

# Print Rust version.
rustc --version
