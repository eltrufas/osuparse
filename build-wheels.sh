#!/bin/bash
set -ex

mkdir ~/rust-installer
curl -sL https://static.rust-lang.org/rustup.sh -o ~/rust-installer/rustup.sh
sh ~/rust-installer/rustup.sh -y
source $HOME/.cargo/env

cd /io/py_osuparse

for PYBIN in /opt/python/{cp27-cp27m,cp27-cp27mu,cp35-cp35m,cp36-cp36m,cp37-cp37m}/bin; do
    export PYTHON_SYS_EXECUTABLE="$PYBIN/python"

    "${PYBIN}/pip" install -U setuptools wheel setuptools-rust
    "${PYBIN}/python" setup.py bdist_wheel
done

/opt/_internal/cpython-3.6.8/bin/pip install auditwheel


for whl in dist/*.whl; do
    auditwheel repair "$whl" -w dist/
done
