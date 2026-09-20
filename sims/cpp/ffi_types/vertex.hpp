#pragma once

#include <ffi_types/vec.hpp>

#include "util/colors.hpp"

struct Vertex {
    Vec3 position;
    Vec2 uv;
    Color color;
    Vec4 normal;
};
