#pragma once

#include <ffi_types/vec.hpp>

constexpr float det_2d(const Vec2 matrix[2]) { return matrix[0].x * matrix[1].y - matrix[0].y * matrix[1].x; }
