#!/bin/bash -x

# This script is called from $GIT_ROOT/.github/workflows/build-test.yml
# This script is called while in $GIT_ROOT/chik-blockchain of chiklisp

. ./venv/bin/activate

python -m pip install --upgrade pip
python -m pip uninstall clvk clvk_rs clvk_tools chiklisp

git clone https://github.com/Chik-Network/clvk.git --branch=main --single-branch
python -m pip install ./clvk

echo "installing clvk_rs via pip"
pip install clvk_rs

echo "installing clvk_tools for clvk tests"

# Ensure clvk_tools is installed from its own repo.
git clone https://github.com/Chik-Network/clvk_tools.git --branch=main --single-branch
python -m pip install ./clvk_tools

# Install chiklisp from the directory above.
python -m pip install ..
