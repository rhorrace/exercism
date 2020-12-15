#include "resistor_color.h"

resistor_band_t const bands[10] = {BLACK, BROWN, RED, ORANGE, YELLOW, GREEN, BLUE, VIOLET, GREY, WHITE};

int color_code(resistor_band_t color) {
  return (int) color;
}

resistor_band_t const* colors() {
  return bands;
}
