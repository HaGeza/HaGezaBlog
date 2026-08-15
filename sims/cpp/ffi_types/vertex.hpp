#pragma once

#include <ffi_types/vec.hpp>

struct Vertex {
    Vec3 position;
    Vec2 uv;
    unsigned char color[4];
    Vec4 normal;
};
