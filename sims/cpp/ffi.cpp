#include "ffi.hpp"

#include "mesh/shape/common.hpp"

static const unsigned int SEMICIRCLE_NUM_SECTIONS = 5;
static constexpr auto SEMICIRCLE = get_semicircle<SEMICIRCLE_NUM_SECTIONS>(3.0, 4.0, 4.0);

extern "C" {
const struct Vec2 *get_baked_semicircle_ptr() { return SEMICIRCLE.data(); }

const unsigned int get_baked_semicircle_num_sections() { return SEMICIRCLE_NUM_SECTIONS; }
}
