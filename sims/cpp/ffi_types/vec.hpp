#pragma once

#include "util/math.hpp"

struct Vec2 {
    float x, y;

    constexpr Vec2 operator+(const Vec2 &other) const { return Vec2{x + other.x, y + other.y}; }

    constexpr Vec2 operator-(const Vec2 &other) const { return Vec2{x - other.x, y - other.y}; }

    constexpr Vec2 operator*(const float scalar) const { return Vec2{x * scalar, y * scalar}; }

    constexpr Vec2 operator/(const float scalar) const { return Vec2{x / scalar, y / scalar}; }

    constexpr bool relative_eq(const Vec2 &other) { return relative_eqf(x, other.x) && relative_eqf(y, other.y); }

    constexpr float length() { return constexpr_sqrt(x * x + y * y); }

    constexpr float distance(const Vec2 &other) { return (*this - other).length(); }
};

struct Vec3 {
    float x, y, z;

    constexpr Vec3 operator*(const float scalar) const { return Vec3{x * scalar, y * scalar, z * scalar}; }

    constexpr Vec3 operator/(const float scalar) const { return Vec3{x / scalar, y / scalar, z / scalar}; }

    constexpr bool relative_eq(const Vec3 &other) {
        return relative_eqf(x, other.x) && relative_eqf(y, other.y) && relative_eqf(z, other.z);
    }
};

struct Vec4 {
    float x, y, z, w;
};

static constexpr Vec2 VEC2_ZERO = {0.0f, 0.0f};
static constexpr Vec3 VEC3_ZERO = {0.0f, 0.0f, 0.0f};
static constexpr Vec4 VEC4_ZERO = {0.0f, 0.0f, 0.0f, 0.0f};
