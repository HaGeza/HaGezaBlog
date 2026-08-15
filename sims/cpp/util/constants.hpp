#pragma once

#include <limits>
#include <numbers>

constexpr float PI = std::numbers::pi_v<float>;
constexpr float FRAC_PI_2 = PI / 2.0;
constexpr float EPSILON = std::numeric_limits<float>::epsilon();
