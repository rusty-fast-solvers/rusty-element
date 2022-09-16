from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="rusty-element",
    description="A library for the evaluation of finite element basis functions.",
    version="0.1.0",
    url="https://github.com/rusty-fast-solvers/rusty-element",
    rust_extensions=[RustExtension(
        "rusty_element._rusty_element", binding=Binding.PyO3, path="../rust/Cargo.toml")],
    packages=["rusty_element"],
    zip_safe=False,
)