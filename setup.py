#!/usr/bin/env python

import sys
from setuptools import setup
from setuptools_rust import RustExtension

setup(
    name="rusty_element",
    description="A library for the evaluation of finite element basis functions.",
    version="0.1.0",
    rust_extensions=[RustExtension("rusty_element.rusty_element")]
)
