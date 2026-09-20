#pragma once
#include <algorithm>
#include <array>
#include <cstdlib>
#include <numbers>
#include <optional>

#include "ffi_types/vec.hpp"
#include "util/constants.hpp"
#include "util/linalg.hpp"
#include "util/math.hpp"

template <std::size_t NumSections>
constexpr std::array<Vec2, NumSections + 1> get_semicircle(float radius, float start_radian, float end_radian) {
    const float radian_step =
        ((end_radian - start_radian) + (start_radian >= end_radian ? 2.0f * std::numbers::pi_v<float> : 0.0f)) /
        static_cast<float>(NumSections);

    std::array<Vec2, NumSections + 1> points{};
    for (std::size_t i = 0; i <= NumSections; ++i) {
        const float theta = start_radian + radian_step * static_cast<float>(i);
        points[i] = Vec2{constexpr_cos(theta) * radius, constexpr_sin(theta) * radius};
    }
    return points;
}

constexpr std::optional<Vec2> intersect(const Vec2 line_a[2], const Vec2 line_b[2]) {
    // We solve the system of linear equations:
    //   x * (a0.y - a1.y) + y (a1.x - a0.x) = -det(a)
    //   x * (b0.y - b1.y) + y (b1.x - b0.x) = -det(b)
    // Using Cramer's rule.

    const Vec2 x_diffs = Vec2{line_a[0].x - line_a[1].x, line_b[0].x - line_b[1].x};
    const Vec2 y_diffs = Vec2{line_a[0].y - line_a[1].y, line_b[0].y - line_b[1].y};
    const Vec2 diffs_mat[2] = {x_diffs, y_diffs};
    float det_ab_transpose = det_2d(diffs_mat);

    if (constexpr_abs(det_ab_transpose) < EPSILON) return std::nullopt;

    Vec2 line_dets = Vec2{det_2d(line_a), det_2d(line_b)};
    const Vec2 mat_x[2] = {line_dets, x_diffs}, mat_y[2] = {line_dets, y_diffs};

    return std::make_optional(Vec2{det_2d(mat_x) / det_ab_transpose, det_2d(mat_y) / det_ab_transpose});
}

template <std::size_t NumSections>
constexpr std::array<Vec2, NumSections + 1> get_quadratic_bezier(const Vec2 pts[3]) {
    std::array<Vec2, NumSections + 1> curve;
    for (size_t pt_ind = 0; pt_ind <= NumSections; ++pt_ind) {
        float ratio = static_cast<float>(pt_ind) / NumSections;
        curve[pt_ind] = (pts[0] * (1.0f - ratio) + pts[1] * ratio) * (1.0f - ratio) +
                        (pts[1] * (1.0f - ratio) + pts[2] * ratio) * ratio;
    }
    return curve;
}

template <std::size_t NumSectionsA, std::size_t NumSectionsB>
constexpr std::array<Vec2, NumSectionsA + NumSectionsB + 2> combine_profiles(
    const std::array<Vec2, NumSectionsA + 1> profile_a, const std::array<Vec2, NumSectionsB + 1> profile_b) {
    constexpr std::array<Vec2, NumSectionsA + NumSectionsB + 2> profile;

    size_t pt_ind = 0;
    for (Vec2 pt : profile_a) profile[pt_ind++] = pt;
    for (Vec2 pt : profile_b) profile[pt_ind++] = pt;
    return profile;
}
