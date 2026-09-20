#pragma once

#include "ffi_types/vec.hpp"

constexpr bool is_non_empty_triangle(Vec3 pos_a, Vec3 pos_b, Vec3 pos_c) {
    return !pos_a.relative_eq(pos_b) && !pos_a.relative_eq(pos_c) && !pos_b.relative_eq(pos_c);
}
