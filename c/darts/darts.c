#include "darts.h"

uint8_t score(coordinate_t coord) {
  float distance = sqrt(pow(coord.x, 2) + pow(coord.y, 2));

  if(distance <= 1) return 10;

  if(distance <= 5) return 5;

  if(distance <= 10) return 1;

  return 0;
}
