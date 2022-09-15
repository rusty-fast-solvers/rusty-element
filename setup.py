#!/usr/bin/env python

import sys
from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="rusty-element",
    description="A library for the evaluation of finite element basis functions.",
    version="0.1.0",
    url="https://github.com/rusty-fast-solvers/rusty-element",
    rust_extensions=[RustExtension("rusty_element._internal")]
    packages=["rusty_element"],
    zip_safe=False,
)
