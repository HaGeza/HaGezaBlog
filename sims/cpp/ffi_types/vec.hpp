#pragma once

struct Vec2 {
    float x, y;

    constexpr Vec2 operator*(const float scalar) const { return Vec2(x * scalar, y * scalar); }
    constexpr Vec2 operator/(const float scalar) const { return Vec2(x / scalar, y / scalar); }
};

struct Vec3 {
    float x, y, z;
};

struct Vec4 {
    float x, y, z, w;
};
