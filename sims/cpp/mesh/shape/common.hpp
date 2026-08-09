struct Vec2 {
  float x, y;
};

const struct Vec2 *get_semicircle(float radius, float start_radian,
                                  unsigned int num_sections);