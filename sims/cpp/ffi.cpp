#include "ffi.hpp"

#include "mesh/models/light_bulb.hpp"
#include "util/constants.hpp"

constexpr float HEAD_RADIANS = 3.0f * FRAC_PI_4;
constexpr float HEAD_RADIUS = 2.0f;

constexpr float BOTTOM_HALF_WIDTH = HEAD_RADIUS / 2.5f;
constexpr float BOTTOM_HEIGHT = BOTTOM_HALF_WIDTH * 1.5f;
constexpr float BOTTOM_START_Y = -HEAD_RADIUS - BOTTOM_HEIGHT;

constexpr size_t HEAD_NUM_SECTIONS = 20;
constexpr size_t NECK_NUM_SECTIONS = HEAD_NUM_SECTIONS / 5;
constexpr size_t NUM_RINGS = HEAD_NUM_SECTIONS + NECK_NUM_SECTIONS;

static constexpr auto LIGHT_BULB_MESH_DATA =
    create_light_bulb_mesh_data<NECK_NUM_SECTIONS, HEAD_NUM_SECTIONS, NUM_RINGS>(
        LightBulbTopProfileParams{
            HEAD_RADIANS,
            HEAD_RADIUS,
            BOTTOM_HALF_WIDTH,
        },
        LightBulbBottomProfileParams{
            BOTTOM_START_Y,
            BOTTOM_HALF_WIDTH,
            BOTTOM_HEIGHT,
        });

extern "C" {
const struct MeshData get_baked_light_bulb_mesh_data() { return LIGHT_BULB_MESH_DATA; }
}
