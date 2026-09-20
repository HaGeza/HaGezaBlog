#pragma once

#include <ffi_types/vertex.hpp>

struct MeshData {
    unsigned int vertex_count;
    const Vertex *vertices;
    unsigned int index_count;
    const unsigned short *indices;
};
