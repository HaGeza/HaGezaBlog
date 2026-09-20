#pragma once
#include <numbers>
#include <stdexcept>

#include "util/constants.hpp"

constexpr float pi = std::numbers::pi_v<float>;

constexpr float normalize_angle(float x) {
    while (x > pi) x -= 2.0f * pi;
    while (x < -pi) x += 2.0f * pi;
    return x;
}

static constexpr int TAYLOR_STEPS = 10;

// Compile-time sine approximation via Taylor series
constexpr float constexpr_sin(float x) {
    x = normalize_angle(x);
    float term = x;
    float sum = x;
    for (int i = 1; i <= TAYLOR_STEPS; ++i) {
        term *= -x * x / static_cast<float>((2 * i) * (2 * i + 1));
        sum += term;
    }
    return sum;
}

constexpr float constexpr_cos(float x) { return constexpr_sin(pi * 0.5f - x); }

constexpr float constexpr_abs(float x) { return x < 0.0f ? -x : x; }

constexpr bool relative_eqf(float a, float b) { return constexpr_abs(a - b) < EPSILON; }

constexpr float constexpr_sqrt(float x) {
    if (x < 0.0f) throw std::invalid_argument("sqrt of negative number");
    for (float guess = x < 1.0f ? 1.0f : x;;) {
        float next = 0.5f * (guess + x / guess);
        if (relative_eqf(next, guess)) return next;
        guess = next;
    }
}
