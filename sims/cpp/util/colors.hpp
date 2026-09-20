#pragma once

struct Color {
    unsigned char red;
    unsigned char green;
    unsigned char blue;
    unsigned char alpha;
};

constexpr Color WHITE = Color{255, 255, 255, 255};
