#pragma once
#include <numbers>

constexpr float pi = std::numbers::pi_v<float>;

consteval float normalize_angle(float x) {
    while (x > pi) x -= 2.0f * pi;
    while (x < -pi) x += 2.0f * pi;
    return x;
}

static constexpr int TAYLOR_STEPS = 10;

// Compile-time sine approximation via Taylor series
consteval float consteval_sin(float x) {
    x = normalize_angle(x);
    float term = x;
    float sum = x;
    for (int i = 1; i <= TAYLOR_STEPS; ++i) {
        term *= -x * x / static_cast<float>((2 * i) * (2 * i + 1));
        sum += term;
    }
    return sum;
}

consteval float consteval_cos(float x) { return consteval_sin(pi * 0.5f - x); }
