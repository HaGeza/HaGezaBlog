#pragma once
#include <array>
#include <stdexcept>

#include "ffi_types/mesh_data.hpp"
#include "mesh/shape/common.hpp"
#include "util/constants.hpp"

struct LightBulbGlassProfileParams {
    unsigned int glass_num_sections;
    float glass_radians;
    float glass_radius;
    float cap_half_width;
};

template <std::size_t NumSections>
consteval Vec2 *create_light_bulb_glass_profile_pts(LightBulbGlassProfileParams params) {
    if (params.glass_num_sections < 2) {
        throw std::invalid_argument("too few glass sections, cannot create semicircle");
    }
    if (params.glass_radians <= FRAC_PI_2) {
        throw std::invalid_argument("too small glass angle, glass and cap profiles wouldn't intersect below glass");
    }
    if (params.glass_radians >= PI) {
        throw std::invalid_argument("too large glass angle, semicircle wouldn't fit in profile");
    }

    std::array<Vec2, NumSections + 1> semicircle =
        get_semicircle(params.glass_radius, FRAC_PI_2 - params.glass_radians, FRAC_PI_2);

    if (semicircle[0].x <= 0.0 || semicircle[0].y >= 0.0 || semicircle[1].x <= 0.0 || semicircle[1].y >= 0.0) {
        // The last section of the glass bulb profile should be in the fourth quarter.
        // If it isn't and `theta` is in the correct range, the resolution is not high enough
        throw std::invalid_argument("glass and cap lines don't intersect");
    }

    Vec2 connection_c = semicircle[1];
    // Vec2 glass_line = [&connection_c, &semicircle[0] ];
    // let cap_line = [&vec2(params.cap_half_width, 0.), &vec2(params.cap_half_width, 1.) ];
    // let Some(connection_b) = intersect(glass_line, cap_line) else { throw std::invalid_argument("NoIntersection"); };
}

consteval MeshData create_light_bulb_mesh_data {
    const Vec2 *glass_profile_pts = create_light_bulb_glass_profile_pts();
}
