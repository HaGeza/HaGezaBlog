#pragma once

#include <array>

#include "ffi_types/vec.hpp"
#include "ffi_types/vertex.hpp"
#include "util/colors.hpp"
#include "util/constants.hpp"
#include "util/trig.hpp"

template <std::size_t NumSections, std::size_t NumRings>
constexpr std::array<Vertex, (NumSections + 1) * NumRings> create_lathe_vertices(
    const std::array<Vec2, NumSections + 1> profile) {
    std::array<Vertex, (NumSections + 1) * NumRings> vertices;
    for (size_t ring = 0; ring < NumRings; ++ring) {
        for (size_t pt_ind = 0; pt_ind <= NumSections; ++pt_ind) {
            Vec2 &pt = profile[pt_ind];
            float theta = static_cast<float>(ring) / static_cast<float>(NumRings) * 2.0 * PI;
            Vec3 position = Vec3{pt.x * consteval_cos(theta), pt.y, pt.x * consteval_sin(theta)};
            vertices[ring * (NumSections + 1) + pt_ind] = Vertex{
                position,
                VEC2_ZERO,  // uv
                WHITE,
                VEC4_ZERO,  // normal
            };
        }
    }
    return vertices;
}

template <std::size_t NumSections, std::size_t NumRings>
constexpr std::array<unsigned short, (NumSections + 1) * NumRings> create_lathe_indices(
    const std::array<unsigned short, NumSections + 1> profile,
    std::array<Vertex, (NumSections + 1) * NumRings> vertices) {
    std::array<unsigned short, (NumSections + 1) * NumRings> indices;
    size_t ind = 0;

    for (size_t ring = 0; ring < NumRings; ++ring) {
        for (size_t pt_ind = 0; pt_ind <= NumSections; ++pt_ind) {
            size_t top_right = ring * profile.size() + pt_ind;
            size_t bot_right = ring * profile.size() + pt_ind + 1;

            size_t next_ring = (ring + 1) % NumRings;
            size_t top_left = next_ring * profile.size() + pt_ind;
            size_t bot_left = next_ring * profile.size() + pt_ind + 1;

            // Add non-empty new faces in counterclockwise vertex order:
            if (is_non_empty_triangle(vertices[top_left].position, vertices[bot_left].position,
                                      vertices[bot_right].position)) {
                indices[ind++] = top_left;
                indices[ind++] = bot_left;
                indices[ind++] = top_right;
            }
            if (is_non_empty_triangle(vertices[top_left].position, vertices[bot_right].position,
                                      vertices[top_right].position)) {
                indices[ind++] = top_left;
                indices[ind++] = bot_right;
                indices[ind++] = top_right;
            }
        }
    }
}
