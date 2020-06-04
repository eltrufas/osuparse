from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="osuparse",
    version="2.0.1",
    rust_extensions=[RustExtension("osuparse.osuparse",
                     binding=Binding.RustCPython)],
    packages=["osuparse"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
