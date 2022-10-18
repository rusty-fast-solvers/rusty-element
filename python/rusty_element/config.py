"""Configure FFI"""

from .rusty_element import lib as _local_lib
from .rusty_element import ffi as _local_ffi

_lib = None
_ffi = None


def get_lib():
    """Return lib."""
    return _lib


def get_ffi():
    """Return ffi."""
    return _ffi


def config(lib, ffi):
    """Set the lib and ffi libraries."""
    from rusty_cffi.config import config as rusty_cffi_config
    global _lib, _ffi

    _lib = lib
    _ffi = ffi

    # Set lib and ffi of dependencies
    rusty_cffi_config(lib, ffi)


config(_local_lib, _local_ffi)
