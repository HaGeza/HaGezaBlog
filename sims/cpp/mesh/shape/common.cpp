#include "common.hpp"

#include <cmath>
#include <numbers>
#include <vector>

template <std::size_t NumSections>
constexpr std::array<Vec2, NumSections + 1>
_get_semicircle(float radius, float start_radian, float end_radian) {
  constexpr float pi = std::numbers::pi_v<float>;
  const float radian_step = ((end_radian - start_radian) +
                             (start_radian >= end_radian ? 2.0f * pi : 0.0f)) /
                            static_cast<float>(NumSections);

  std::array<Vec2, NumSections + 1> points{};

  for (std::size_t i = 0; i <= NumSections; ++i) {
    const float theta = start_radian + radian_step * static_cast<float>(i);
    points[i] = Vec2{std::cos(theta) * radius, std::sin(theta) * radius};
  }

  return points;
}

static const auto SEMICIRCLE = _get_semicircle<5>(3.0, 4.0, 4.0);

extern "C" {
const struct Vec2 *get_semicircle() { return SEMICIRCLE.data(); }
}