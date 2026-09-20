#pragma once

#include <cmath>

#include "util/constants.hpp"
struct Vec2 {
    float x, y;

    constexpr Vec2 operator+(const Vec2 &other) const { return Vec2{x + other.x, y + other.y}; }

    constexpr Vec2 operator-(const Vec2 &other) const { return Vec2{x - other.x, y - other.y}; }

    constexpr Vec2 operator*(const float scalar) const { return Vec2{x * scalar, y * scalar}; }

    constexpr Vec2 operator/(const float scalar) const { return Vec2{x / scalar, y / scalar}; }

    bool relative_eq(const Vec2 &other) { return (fabsf(x - other.x) < EPSILON) && (fabsf(y - other.y) < EPSILON); }

    float length() { return sqrtf(x * x + y * y); }

    float distance(const Vec2 &other) { return (*this - other).length(); }
};

struct Vec3 {
    float x, y, z;

    constexpr Vec3 operator*(const float scalar) const { return Vec3{x * scalar, y * scalar, z * scalar}; }

    constexpr Vec3 operator/(const float scalar) const { return Vec3{x / scalar, y / scalar, z / scalar}; }

    bool relative_eq(const Vec3 &other) {
        return (fabsf(x - other.x) < EPSILON) && (fabsf(y - other.y) < EPSILON) && (fabsf(z - other.z) < EPSILON);
    }
};

struct Vec4 {
    float x, y, z, w;
};

static constexpr Vec2 VEC2_ZERO = {0.0f, 0.0f};
static constexpr Vec3 VEC3_ZERO = {0.0f, 0.0f, 0.0f};
static constexpr Vec4 VEC4_ZERO = {0.0f, 0.0f, 0.0f, 0.0f};
