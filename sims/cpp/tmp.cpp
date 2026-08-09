#include "tmp.hpp"

#include <algorithm>
#include <array>
#include <concepts>

// This works 100% fine with .cpp_link_stdlib(None)!
constexpr auto generate_data() {
  std::array<float, 4> data = {3.0f, 1.0f, 4.0f, 2.0f};
  std::sort(data.begin(), data.end());
  return data;
}
