"""Data structures for cell descriptions."""

import rusty_cffi
from .config import get_ffi, get_lib


class ReferenceCell:
    """Reference cell descriptions."""

    def __init__(self, ptr):
        """A new reference cell from a C pointer."""
        self._ptr = ptr

    @property
    def dim(self):
        """Return dimension."""
        return get_lib().reference_cell_container_get_dim(self._ptr)

    @property
    def vertices(self):
        """Return vertices."""
        container = rusty_cffi.RustyDataContainer(
            get_lib().reference_cell_container_get_vertices(self._ptr)
        )
        return container.data.copy()

    @property
    def edges(self):
        """Return edges."""
        container = rusty_cffi.RustyDataContainer(
            get_lib().reference_cell_container_get_edges(self._ptr)
        )
        return container.data.copy()

    @property
    def faces(self):
        """Return faces."""
        container = rusty_cffi.RustyDataContainer(
            get_lib().reference_cell_container_get_faces(self._ptr)
        )
        return container.data.copy()

    @property
    def faces_nvertices(self):
        """Number of vertices adjacent to each face."""
        container = rusty_cffi.RustyDataContainer(
            get_lib().reference_cell_container_get_faces_nvertices(self._ptr)
        )
        return container.data.copy()

    def entity_count(self, dim):
        """Number of entities of given dimension."""
        if dim > 3:
            raise ValueError(f"`dim` = {dim}, but `dim` < 4 required.")
        return get_lib().reference_cell_container_get_entity_count(dim, self._ptr)

    @property
    def vertex_count(self):
        """Number of vertices."""
        return get_lib().reference_cell_container_get_vertex_count(self._ptr)

    @property
    def edge_count(self):
        """Number of edges."""
        return get_lib().reference_cell_container_get_edge_count(self._ptr)

    @property
    def face_count(self):
        """Number of faces."""
        return get_lib().reference_cell_container_get_face_count(self._ptr)

    @property
    def volume_count(self):
        """Number of volumes."""
        return get_lib().reference_cell_container_get_volume_count(self._ptr)

    def connectivity(self, entity_dim, entity_number, connected_dim):
        """Connectivity."""

        container = rusty_cffi.RustyDataContainer(
            get_lib().reference_cell_container_get_connectivity(
                entity_dim, entity_number, connected_dim, self._ptr
            )
        )
        return container.data.copy()

    @property
    def cell_type(self):
        """Cell type."""
        return get_lib().reference_cell_container_get_cell_type(self._ptr)

    def __del__(self):
        """Call destructor."""
        get_lib().reference_cell_container_destroy(self._ptr)

    @classmethod
    def interval(cls):
        """New from interval."""
        return cls(get_lib().reference_cell_container_new_from_interval())

    @classmethod
    def triangle(cls):
        """New from triangle."""
        return cls(get_lib().reference_cell_container_new_from_triangle())

    @classmethod
    def quadrilateral(cls):
        """New from quadrilateral."""
        return cls(get_lib().reference_cell_container_new_from_quadrilateral())

    @classmethod
    def tetrahedron(cls):
        """New from tetrahedron."""
        return cls(get_lib().reference_cell_container_new_from_tetrahedron())

    @classmethod
    def hexahedron(cls):
        """New from hexahedron."""
        return cls(get_lib().reference_cell_container_new_from_hexahedron())

    @classmethod
    def pyramid(cls):
        """New from pyramid."""
        return cls(get_lib().reference_cell_container_new_from_pyramid())

    @classmethod
    def prism(cls):
        """New from prism."""
        return cls(get_lib().reference_cell_container_new_from_prism())
