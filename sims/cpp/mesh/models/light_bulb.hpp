#pragma once
#include <ranges>
#include <stdexcept>

#include "ffi_types/mesh_data.hpp"
#include "mesh/lathe_mesh.hpp"
#include "shape/common.hpp"
#include "util/constants.hpp"

//          ******          +---+------+
//       ************           |      |
//     ****************         |      |
//    ******************        | HEAD |
//    ******************        |      |
//    ******************        |      |
//     ****************         |      |
//      **************      +---+      | TOP
//       ************           |      |
//       ************           | NECK |
//        **********            |      |
//        **********            |      |
//        **********        +---+------+
//        **********            |
//        **********            | BOTTOM
//         ********             |
//          ******              |
//           ****           +---+

/// Parameters for constructing the profile of the top of the light bulb
struct LightBulbTopProfileParams {
    float head_semicircle_radians;
    float head_semicircle_radius;
    float bottom_half_width;
};

/// Create the profile of the the top (glass part) of the light bulb
template <std::size_t NeckNumSections, std::size_t HeadNumSections>
consteval std::array<Vec2, NeckNumSections + HeadNumSections + 1> create_light_bulb_top_profile(
    LightBulbTopProfileParams params) {
    if (HeadNumSections < 2) {
        throw std::invalid_argument("too few head semicircle sections, cannot create semicircle");
    }
    if (params.head_semicircle_radians <= FRAC_PI_2) {
        throw std::invalid_argument(
            "too small head semicircle angle, head and collar profiles don't intersect below head");
    }
    if (params.head_semicircle_radians >= PI) {
        throw std::invalid_argument("too large head semicircle angle, semicircle wouldn't fit in profile");
    }

    auto head = get_semicircle<HeadNumSections>(params.head_semicircle_radius,
                                                FRAC_PI_2 - params.head_semicircle_radians, FRAC_PI_2);
    if (head[0].x <= 0.0 || head[0].y >= 0.0 || head[1].x <= 0.0 || head[1].y >= 0.0) {
        // The last section of the head profile should be in the fourth quarter.
        // If it isn't and `theta` is in the correct range, the resolution is not high enough
        throw std::invalid_argument("head and collar profiles don't intersect");
    }

    Vec2 neck_c = head[1];
    Vec2 head_first_section[2] = {neck_c, head[0]};
    Vec2 collar_section[2] = {Vec2{params.bottom_half_width, 0.0f}, Vec2{params.bottom_half_width, 1.0f}};

    std::optional<Vec2> neck_b_opt = intersect(head_first_section, collar_section);
    if (!neck_b_opt) {
        throw std::invalid_argument("head and collar profiles don't intersect");
    };
    Vec2 neck_b = neck_b_opt.value();
    Vec2 neck_a = Vec2{neck_b.x, neck_b.y - neck_c.distance(neck_b)};
    auto neck = get_quadratic_bezier<NeckNumSections>(neck_a, neck_b, neck_c);

    return combine_profiles(neck, head | std::views::drop(1));
}

struct LightBulbBottomProfileParams {
    float border_y;
    float half_width;
    float height;
};

/// Create the profile of the bottom (non-glass part) of the light bulb
consteval std::array<Vec2, 2> create_light_bulb_bottom_profile(LightBulbBottomProfileParams params) {
    return std::array<Vec2, 2>({
        Vec2{params.half_width, params.border_y},
        Vec2{params.half_width, params.border_y + params.height},
    });
}

template <std::size_t NeckNumSections, std::size_t HeadNumSections, std::size_t NumRings>
consteval MeshData create_light_bulb_mesh_data(LightBulbTopProfileParams top_profile_params,
                                               LightBulbBottomProfileParams bottom_profile_params) {
    auto profile =
        combine_profiles(create_light_bulb_bottom_profile(bottom_profile_params),
                         create_light_bulb_top_profile<NeckNumSections, HeadNumSections>(top_profile_params));

    auto vertices = create_lathe_vertices<1 + NeckNumSections + HeadNumSections, NumRings>(profile);
    auto indices = create_lathe_indices(profile, vertices);

    return MeshData{
        static_cast<unsigned int>(vertices.size()),
        vertices.data,
        static_cast<unsigned int>(indices.size()),
        indices.data,
    };
}
