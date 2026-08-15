#pragma once

#include <ffi_types/vertex.hpp>

struct MeshData {
    unsigned int vertex_count;
    Vertex *vertices;
    unsigned int index_count;
    unsigned short *indices;
};
