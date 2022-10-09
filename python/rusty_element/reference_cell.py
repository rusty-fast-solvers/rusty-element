"""Data structures for cell descriptions."""

from .rusty_element import lib, ffi
import rusty_cffi

rusty_cffi.configure(lib, ffi)


class ReferenceCell:
    """Reference cell descriptions."""

    def __init__(self, ptr):
        """A new reference cell from a C pointer."""

        self._ptr = ptr

    @property
    def dim(self):
        """Return dimension."""
        return lib.reference_cell_container_get_dim(self._ptr)

    @property
    def vertices(self):
        """Return vertices."""
        container = rusty_cffi.RustyDataContainer(
            lib.reference_cell_container_get_vertices(self._ptr)
        )
        return container.data.copy()

    def __del__(self):
        """Call destructor."""
        lib.reference_cell_container_destroy(self._ptr)

    @classmethod
    def interval(cls):
        """New from interval."""
        return cls(lib.reference_cell_container_new_from_interval())
