#include "ffi.hpp"

#include "mesh/shape/common.hpp"

static constexpr float PI = std::numbers::pi_v<float>;
static constexpr float FRAC_PI_2 = PI / 2.0;
static constexpr unsigned int SEMICIRCLE_NUM_SECTIONS = 20;
static constexpr float GLASS_RADIUS = 2.0;
static constexpr float GLASS_RADIANS = 3.0 * PI / 4.0;

static constexpr auto SEMICIRCLE =
    get_semicircle<SEMICIRCLE_NUM_SECTIONS>(GLASS_RADIUS, FRAC_PI_2 - GLASS_RADIANS, FRAC_PI_2);

extern "C" {
const struct Vec2 *get_baked_semicircle_ptr() { return SEMICIRCLE.data(); }
const unsigned int get_baked_semicircle_num_sections() { return SEMICIRCLE_NUM_SECTIONS; }
}
