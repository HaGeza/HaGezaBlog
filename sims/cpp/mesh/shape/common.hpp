#pragma once
#include <numbers>
#include <vector>

#include "ffi_types/vec.hpp"
#include "util/trig.hpp"

template <std::size_t NumSections>
consteval std::array<Vec2, NumSections + 1> get_semicircle(float radius, float start_radian, float end_radian) {
    constexpr float pi = std::numbers::pi_v<float>;
    const float radian_step = ((end_radian - start_radian) + (start_radian >= end_radian ? 2.0f * pi : 0.0f)) /
                              static_cast<float>(NumSections);

    std::array<Vec2, NumSections + 1> points{};

    for (std::size_t i = 0; i <= NumSections; ++i) {
        const float theta = start_radian + radian_step * static_cast<float>(i);
        points[i] = Vec2{consteval_cos(theta) * radius, consteval_sin(theta) * radius};
    }

    return points;
}
