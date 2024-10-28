#include "resistor_color.h"

int color_code(resistor_band_t color) {
  return (int) color;
}


resistor_band_t* colors() {
  resistor_band_t* array = (resistor_band_t*) malloc(sizeof(resistor_band_t) * 10);

  for(int i = BLACK;i <= WHITE;++i) {
    array[i] = (resistor_band_t) i;
  }

  return array;
}
